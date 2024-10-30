import 'dart:async';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_app/bridge_generated.dart';
import 'package:flutter_app/components/game_screen.dart';
import 'package:flutter_app/services/rust_app_service.dart';

class GamePage extends StatefulWidget {
  final Uint8List romData;
  final String gameName;

  GamePage({required this.romData, required this.gameName});

  @override
  _GamePageState createState() => _GamePageState();
}

class _GamePageState extends State<GamePage> {
  final RustAppImpl api = RustAppService.instance;
  late Timer _timer;
  Uint32List _frameBuffer = Uint32List(160 * 144);
  bool _isRunning = false;
  bool _isLoaded = false;

  Map<String, bool> _buttonStates = {
    "Up": true,
    "Down": true,
    "Left": true,
    "Right": true,
    "A": true,
    "B": true,
    "Start": true,
    "Select": true,
  };

  @override
  void initState() {
    super.initState();
    _loadGame();
  }

  Future<void> _loadGame() async {
    try {
      await api.load(romData: widget.romData);
      setState(() {
        _isLoaded = true;
      });
    } catch (e) {
      print("Błąd podczas ładowania ROM: $e");
    }
  }

  void _startEmulator() {
    if (!_isLoaded) {
      print("ROM nie został załadowany.");
      return;
    }

    setState(() {
      _isRunning = true;
    });

    _startGameLoop();
  }

  void _startGameLoop() {
    const frameDuration = Duration(milliseconds: 16);
    _timer = Timer.periodic(frameDuration, (timer) async {
      if (_isRunning) {
        try {
          Uint8List buttonStates = _getButtonStates();

          final frame = await api.render();

          await api.setButtons(buttonStates: buttonStates);

          if (frame != null) {
            setState(() {
              _frameBuffer = Uint32List.fromList(frame);
            });
          }

          await api.handleVblank();
        } catch (e) {
          print('Błąd podczas renderowania klatki: $e');
        }
      }
    });
  }

  void _handleButtonPress(String button) {
    setState(() {
      _buttonStates[button] = false;
    });
  }

  void _handleButtonRelease(String button) {
    setState(() {
      _buttonStates[button] = true;
    });
  }

  Uint8List _getButtonStates() {
    return Uint8List.fromList(
      _buttonStates.values.map((state) => state ? 1 : 0).toList(),
    );
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    String gameName = widget.gameName;

    return Scaffold(
      body: Stack(
        children: [
          Positioned.fill(
            child: Image.asset(
              'assets/backgrounds/app_background.png',
              fit: BoxFit.cover,
            ),
          ),
          Column(
            children: [
              SizedBox(height: 60),
              Text(
                gameName,
                style: TextStyle(
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 2.0,
                  color: Colors.white,
                ),
              ),
              SizedBox(height: 20),
              Padding(
                padding: const EdgeInsets.only(top: 10.0),
                child: GameScreen(frameBuffer: _frameBuffer),
              ),
              SizedBox(height: 20),
              if (_isLoaded && !_isRunning)
                ElevatedButton(
                  onPressed: _startEmulator,
                  child: Text(
                    'Start Emulator',
                    style: TextStyle(
                      color: Colors.white,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.white.withOpacity(0.3),
                  ),
                ),
            ],
          ),
          Positioned(
            top: 30,
            left: 10,
            child: IconButton(
              icon: Icon(Icons.arrow_back, size: 30, color: Colors.white),
              onPressed: () {
                api.unload();
                Navigator.of(context).pop();
              },
            ),
          ),
          Positioned(
            bottom: 40,
            left: MediaQuery.of(context).size.width / 4,
            right: MediaQuery.of(context).size.width / 4,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("Select"),
                  onTapUp: (_) => _handleButtonRelease("Select"),
                  onTapCancel: () => _handleButtonRelease("Select"),
                  child: _buildTransparentButton("Select"),
                ),
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("Start"),
                  onTapUp: (_) => _handleButtonRelease("Start"),
                  onTapCancel: () => _handleButtonRelease("Start"),
                  child: _buildTransparentButton("Start"),
                ),
              ],
            ),
          ),
          Positioned(
              bottom: 200,
              right: 20,
              child: Column(children: [
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("A"),
                  onTapUp: (_) => _handleButtonRelease("A"),
                  onTapCancel: () => _handleButtonRelease("A"),
                  child: _buildCircularButton("A"),
                ),
              ])),
          Positioned(
            bottom: 120,
            right: 70,
            child: Column(
              children: [
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("B"),
                  onTapUp: (_) => _handleButtonRelease("B"),
                  onTapCancel: () => _handleButtonRelease("B"),
                  child: _buildCircularButton("B"),
                ),
              ],
            ),
          ),
          Positioned(
            bottom: 100,
            left: 10,
            child: Column(
              children: [
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("Up"),
                  onTapUp: (_) => _handleButtonRelease("Up"),
                  onTapCancel: () => _handleButtonRelease("Up"),
                  child: _buildArrowButton(Icons.arrow_upward, "Up"),
                ),
                Row(
                  children: [
                    GestureDetector(
                      onTapDown: (_) => _handleButtonPress("Left"),
                      onTapUp: (_) => _handleButtonRelease("Left"),
                      onTapCancel: () => _handleButtonRelease("Left"),
                      child: _buildArrowButton(Icons.arrow_back, "Left"),
                    ),
                    SizedBox(width: 30),
                    GestureDetector(
                      onTapDown: (_) => _handleButtonPress("Right"),
                      onTapUp: (_) => _handleButtonRelease("Right"),
                      onTapCancel: () => _handleButtonRelease("Right"),
                      child: _buildArrowButton(Icons.arrow_forward, "Right"),
                    ),
                  ],
                ),
                GestureDetector(
                  onTapDown: (_) => _handleButtonPress("Down"),
                  onTapUp: (_) => _handleButtonRelease("Down"),
                  onTapCancel: () => _handleButtonRelease("Down"),
                  child: _buildArrowButton(Icons.arrow_downward, "Down"),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTransparentButton(String label) {
    return SizedBox(
      width: 70,
      height: 30,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.3),
          borderRadius: BorderRadius.circular(20),
        ),
        child: Center(
          child: Text(
            label,
            style: TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.bold,
              fontSize: 14,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildCircularButton(String label) {
    return SizedBox(
      width: 80,
      height: 80,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.3),
          shape: BoxShape.circle,
        ),
        child: Center(
          child: Text(
            label,
            style: TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.bold,
              fontSize: 14,
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildArrowButton(IconData icon, String direction) {
    return SizedBox(
      width: 75,
      height: 75,
      child: Container(
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.3),
          shape: BoxShape.circle,
        ),
        child: Center(
          child: Icon(
            icon,
            color: Colors.white,
            size: 20,
          ),
        ),
      ),
    );
  }
}
