import 'package:flutter/material.dart';
import 'package:auto_route/auto_route.dart';
import 'package:flutter_app/router/app_router.dart';

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('GameBoy Emulator'),
      ),
      body: Center(
        child: ElevatedButton(
          onPressed: () {
            context.router.push(GamesRoute());
          },
          child: Text('Go to Games'),
        ),
      ),
    );
  }
}
