import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_app/components/game_screen.dart';

class GamePage extends StatelessWidget {
  final String gamePath;

  GamePage({required this.gamePath});

  @override
  Widget build(BuildContext context) {
    String gameName =
        gamePath.split('/').last.replaceAll('.gb', '').toUpperCase();

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
                child: GameScreen(),
              ),
            ],
          ),
          Positioned(
            top: 30,
            left: 10,
            child: IconButton(
              icon: Icon(Icons.arrow_back, size: 30),
              onPressed: () =>
                context.router.pop();
            ),
          ),
        ],
      ),
    );
  }
}
