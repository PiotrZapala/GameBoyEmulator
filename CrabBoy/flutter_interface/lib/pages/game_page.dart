import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_interface/bridge_generated.dart';
import 'package:flutter_interface/components/game_screen.dart';
import 'package:flutter_interface/components/speed_toggle_button.dart';
import 'package:flutter_interface/services/rust_core_service.dart';
import 'package:flutter_interface/components/action_button.dart';
import 'package:path_provider/path_provider.dart';

class GamePage extends StatefulWidget {
  final Uint8List romData;
  final String gameName;
  final Uint8List? ramData;

  GamePage(
      {required this.romData, required this.gameName, required this.ramData});

  @override
  _GamePageState createState() => _GamePageState();
}

class _GamePageState extends State<GamePage> {
  final RustCoreImpl api = RustCoreService.instance;
  Timer? _timer;
  Uint32List _frameBuffer = Uint32List(160 * 144);
  bool _isRunning = false;
  bool _isLoaded = false;
  bool _isDoubleSpeed = false;
  Duration frameDuration = Duration(milliseconds: 16);

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
      await api.load(romData: widget.romData, ramData: widget.ramData);
      setState(() {
        _isLoaded = true;
      });
    } catch (e) {
      print("Błąd podczas ładowania ROM: $e");
    }
  }

  Future<void> _saveGameRam(String gameName, Uint8List ramData) async {
    final storagePath = await _getRomStoragePath();
    final ramFilePath = '$storagePath/$gameName.sav';

    await File(ramFilePath).writeAsBytes(ramData);
  }

  Future<String> _getRomStoragePath() async {
    final directory = await getApplicationDocumentsDirectory();
    return directory.path;
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
    _timer?.cancel();
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
        } catch (e) {
          print('Błąd podczas renderowania klatki: $e');
        }
      }
    });
  }

  void _toggleSpeed() {
    setState(() {
      _isDoubleSpeed = !_isDoubleSpeed;
      frameDuration = _isDoubleSpeed
          ? Duration(milliseconds: 8)
          : Duration(milliseconds: 16);
    });
    _startGameLoop();
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
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    String gameName = widget.gameName;
    final screenWidth = MediaQuery.of(context).size.width;
    final screenHeight = MediaQuery.of(context).size.height;
    bool isLandscape =
        MediaQuery.of(context).orientation == Orientation.landscape;
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
              SizedBox(
                height: isLandscape ? screenWidth * 0.01 : screenHeight * 0.1,
              ),
              if (!isLandscape)
                Text(
                  gameName,
                  style: TextStyle(
                    fontSize: gameName.length > 20 ? 18 : 24,
                    fontWeight: FontWeight.bold,
                    letterSpacing: 2.0,
                    color: Colors.white,
                  ),
                  textAlign: TextAlign.center,
                  softWrap: true,
                  overflow: TextOverflow.visible,
                ),
              Padding(
                padding: const EdgeInsets.only(top: 10.0),
                child: GameScreen(frameBuffer: _frameBuffer),
              ),
              if (_isLoaded && !_isRunning)
                ActionButton(
                  iconPath: 'assets/buttons/startemulator.png',
                  pressedIconPath: 'assets/buttons/startemulator_hover.png',
                  onTapDown: () => _startEmulator(),
                  onTapUp: () => {},
                  onTapCancel: () => {},
                  width: isLandscape ? screenHeight * 0.4 : screenWidth * 0.4,
                  height:
                      isLandscape ? screenWidth * 0.05 : screenHeight * 0.05,
                ),
            ],
          ),
          Positioned(
            top: isLandscape ? screenWidth * 0.04 : screenHeight * 0.04,
            left: isLandscape ? screenHeight * 0.02 : screenWidth * 0.02,
            child: IconButton(
              icon: Icon(Icons.arrow_back, size: 30, color: Colors.white),
              onPressed: () async {
                final ramData = await api.unload();
                if (ramData != null) {
                  await _saveGameRam(gameName, ramData);
                }
                Navigator.of(context).pop();
              },
            ),
          ),
          Positioned(
              top: isLandscape ? screenWidth * 0.05 : screenHeight * 0.05,
              right: isLandscape ? screenHeight * 0.02 : screenWidth * 0.02,
              child: SpeedToggleButton(
                isDoubleSpeed: _isDoubleSpeed,
                toggleSpeed: _toggleSpeed,
                width: isLandscape ? screenHeight * 0.1 : screenWidth * 0.1,
                height: isLandscape ? screenHeight * 0.1 : screenWidth * 0.1,
              )),
          Positioned(
            bottom: isLandscape ? screenWidth * 0.02 : screenHeight * 0.03,
            left: isLandscape ? screenWidth / 5 : screenWidth / 4,
            right: isLandscape ? screenWidth / 5 : screenWidth / 4,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                ActionButton(
                  iconPath: 'assets/buttons/select.png',
                  pressedIconPath: 'assets/buttons/select_hover.png',
                  onTapDown: () => _handleButtonPress("Select"),
                  onTapUp: () => _handleButtonRelease("Select"),
                  onTapCancel: () => _handleButtonRelease("Select"),
                  width:
                      isLandscape ? screenHeight * 0.175 : screenWidth * 0.175,
                  height:
                      isLandscape ? screenWidth * 0.036 : screenHeight * 0.036,
                ),
                ActionButton(
                  iconPath: 'assets/buttons/start.png',
                  pressedIconPath: 'assets/buttons/start_hover.png',
                  onTapDown: () => _handleButtonPress("Start"),
                  onTapUp: () => _handleButtonRelease("Start"),
                  onTapCancel: () => _handleButtonRelease("Start"),
                  width:
                      isLandscape ? screenHeight * 0.175 : screenWidth * 0.175,
                  height:
                      isLandscape ? screenWidth * 0.036 : screenHeight * 0.036,
                ),
              ],
            ),
          ),
          Positioned(
              bottom: isLandscape ? screenWidth * 0.19 : screenHeight * 0.24,
              right: isLandscape ? screenHeight * 0.11 : screenWidth * 0.03,
              child: Column(children: [
                ActionButton(
                  iconPath: 'assets/buttons/A.png',
                  pressedIconPath: 'assets/buttons/A_hover.png',
                  onTapDown: () => _handleButtonPress("A"),
                  onTapUp: () => _handleButtonRelease("A"),
                  onTapCancel: () => _handleButtonRelease("A"),
                  width: isLandscape ? screenHeight * 0.2 : screenWidth * 0.2,
                  height: isLandscape ? screenHeight * 0.2 : screenWidth * 0.2,
                ),
              ])),
          Positioned(
            bottom: isLandscape ? screenWidth * 0.1 : screenHeight * 0.15,
            right: isLandscape ? screenHeight * 0.23 : screenWidth * 0.17,
            child: Column(
              children: [
                ActionButton(
                  iconPath: 'assets/buttons/B.png',
                  pressedIconPath: 'assets/buttons/B_hover.png',
                  onTapDown: () => _handleButtonPress("B"),
                  onTapUp: () => _handleButtonRelease("B"),
                  onTapCancel: () => _handleButtonRelease("B"),
                  width: isLandscape ? screenHeight * 0.2 : screenWidth * 0.2,
                  height: isLandscape ? screenHeight * 0.2 : screenWidth * 0.2,
                ),
              ],
            ),
          ),
          Positioned(
            bottom: isLandscape ? screenWidth * 0.06 : screenHeight * 0.1,
            left: isLandscape ? screenHeight * 0.06 : screenWidth * 0.02,
            child: Column(
              children: [
                ActionButton(
                  iconPath: 'assets/buttons/arrowup.png',
                  pressedIconPath: 'assets/buttons/arrowup_hover.png',
                  onTapDown: () => _handleButtonPress("Up"),
                  onTapUp: () => _handleButtonRelease("Up"),
                  onTapCancel: () => _handleButtonRelease("Up"),
                  width: isLandscape ? screenHeight * 0.22 : screenWidth * 0.22,
                  height:
                      isLandscape ? screenHeight * 0.22 : screenWidth * 0.22,
                ),
                Row(
                  children: [
                    ActionButton(
                      iconPath: 'assets/buttons/arrowleft.png',
                      pressedIconPath: 'assets/buttons/arrowleft_hover.png',
                      onTapDown: () => _handleButtonPress("Left"),
                      onTapUp: () => _handleButtonRelease("Left"),
                      onTapCancel: () => _handleButtonRelease("Left"),
                      width: isLandscape
                          ? screenHeight * 0.22
                          : screenWidth * 0.22,
                      height: isLandscape
                          ? screenHeight * 0.22
                          : screenWidth * 0.22,
                    ),
                    SizedBox(
                      width: isLandscape
                          ? screenHeight * 0.15
                          : screenWidth * 0.15,
                    ),
                    ActionButton(
                      iconPath: 'assets/buttons/arrowright.png',
                      pressedIconPath: 'assets/buttons/arrowright_hover.png',
                      onTapDown: () => _handleButtonPress("Right"),
                      onTapUp: () => _handleButtonRelease("Right"),
                      onTapCancel: () => _handleButtonRelease("Right"),
                      width: isLandscape
                          ? screenHeight * 0.22
                          : screenWidth * 0.22,
                      height: isLandscape
                          ? screenHeight * 0.22
                          : screenWidth * 0.22,
                    ),
                  ],
                ),
                ActionButton(
                  iconPath: 'assets/buttons/arrowdown.png',
                  pressedIconPath: 'assets/buttons/arrowdown_hover.png',
                  onTapDown: () => _handleButtonPress("Down"),
                  onTapUp: () => _handleButtonRelease("Down"),
                  onTapCancel: () => _handleButtonRelease("Down"),
                  width: isLandscape ? screenHeight * 0.22 : screenWidth * 0.22,
                  height:
                      isLandscape ? screenHeight * 0.22 : screenWidth * 0.22,
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
