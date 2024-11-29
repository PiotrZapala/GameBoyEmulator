import 'package:flutter/material.dart';
import 'package:auto_route/auto_route.dart';
import 'package:flutter_interface/router/app_router.dart';
import 'package:flutter_interface/components/action_button.dart';

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
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
          Positioned(
            top: isLandscape ? screenHeight * 0.12 : screenHeight * 0.12,
            left: 0,
            right: 0,
            child: Center(
              child: Image.asset(
                'assets/logo/crabboylogo.png',
                height: isLandscape ? screenWidth * 0.07 : screenHeight * 0.07,
                fit: BoxFit.contain,
              ),
            ),
          ),
          Center(
            child: ActionButton(
              iconPath: 'assets/buttons/gotogames.png',
              pressedIconPath: 'assets/buttons/gotogames_hover.png',
              onTapDown: () => context.router.push(GamesRoute()),
              onTapUp: () => {},
              onTapCancel: () => {},
              width: isLandscape ? screenHeight * 0.5 : screenWidth * 0.5,
              height: isLandscape ? screenWidth * 0.05 : screenHeight * 0.05,
            ),
          ),
        ],
      ),
    );
  }
}
