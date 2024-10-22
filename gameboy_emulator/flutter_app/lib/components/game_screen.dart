import 'dart:typed_data';
import 'package:flutter/material.dart';

class GameScreen extends StatelessWidget {
  final Uint32List frameBuffer;
  GameScreen({required this.frameBuffer});

  final int originalWidth = 160;
  final int originalHeight = 144;

  @override
  Widget build(BuildContext context) {
    double screenWidth = MediaQuery.of(context).size.width;
    double scale = screenWidth * 0.7 / originalWidth;

    double finalWidth = originalWidth * scale;
    double finalHeight = originalHeight * scale;

    return Align(
      alignment: Alignment.topCenter,
      child: Container(
        width: finalWidth,
        height: finalHeight,
        decoration: BoxDecoration(
          color: Colors.black,
          border: Border.all(color: Colors.grey, width: 4),
          borderRadius: BorderRadius.circular(8),
        ),
        child: CustomPaint(
          painter: GamePainter(frameBuffer, scale),
        ),
      ),
    );
  }
}

class GamePainter extends CustomPainter {
  final Uint32List frameBuffer;
  final double scale;

  GamePainter(this.frameBuffer, this.scale);

  @override
  void paint(Canvas canvas, Size size) {
    Paint paint = Paint();

    for (int y = 0; y < 144; y++) {
      for (int x = 0; x < 160; x++) {
        int color = frameBuffer[y * 160 + x];
        paint.color = Color(color);
        canvas.drawRect(
          Rect.fromLTWH(x * scale, y * scale, scale, scale),
          paint,
        );
      }
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}
