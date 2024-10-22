import 'dart:async';
import 'dart:typed_data';
import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_app/bridge_definitions.dart';
import 'package:flutter_app/components/game_screen.dart';
import 'dart:ffi';
import 'dart:io' show Platform;
import 'package:flutter_app/bridge_generated.dart';

class GamePage extends StatefulWidget {
  final Uint8List romData;

  GamePage({required this.romData});

  @override
  _GamePageState createState() => _GamePageState();
}

class _GamePageState extends State<GamePage> {
  late Timer _timer;
  Uint32List _frameBuffer = Uint32List(160 * 144);
  late RustApp api;
  bool _isRunning = false;
  bool _isLoaded = false;

  @override
  void initState() {
    super.initState();
    api = RustAppImpl(Platform.isIOS
        ? DynamicLibrary.process()
        : DynamicLibrary.open('librust_app.so'));
    _loadGame();
  }

  Future<void> _loadGame() async {
    try {
      print("ROM Data Length: ${widget.romData.length}");
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
          final frame = await api.render();
          print('Frame received: $frame');
          if (frame != null) {
            setState(() {
              _frameBuffer = Uint32List.fromList(frame);
            });
          }
        } catch (e) {
          print('Błąd podczas renderowania klatki: $e');
        }
      }
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    String gameName = "Loaded Game";

    return Scaffold(
      body: Stack(
        children: [
          Column(
            children: [
              SizedBox(height: 60),
              Text(
                gameName,
                style: TextStyle(
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                  letterSpacing: 2.0,
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
                  child: Text('Start Emulator'),
                ),
            ],
          ),
          Positioned(
            top: 30,
            left: 10,
            child: IconButton(
                icon: Icon(Icons.arrow_back, size: 30),
                onPressed: () => context.router.pop()),
          ),
        ],
      ),
    );
  }
}
