import 'dart:typed_data';
import 'package:flutter/material.dart';

class GameScreen extends StatelessWidget {
  final Uint32List frameBuffer;
  GameScreen({required this.frameBuffer});

  final int originalWidth = 160;
  final int originalHeight = 144;

  @override
  Widget build(BuildContext context) {
    bool isLandscape =
        MediaQuery.of(context).orientation == Orientation.landscape;

    double screenWidth = isLandscape
        ? MediaQuery.of(context).size.height
        : MediaQuery.of(context).size.width;

    double scale = screenWidth * 0.8 / originalWidth;

    double finalWidth = originalWidth * scale;
    double finalHeight = originalHeight * scale;

    return Center(
      child: Container(
        decoration: BoxDecoration(
          color: Colors.black,
          border: Border.all(color: Colors.grey, width: 4),
          borderRadius: BorderRadius.circular(8),
        ),
        child: Padding(
          padding: const EdgeInsets.all(10.0),
          child: SizedBox(
            width: finalWidth,
            height: finalHeight,
            child: CustomPaint(
              painter: GamePainter(frameBuffer, scale),
            ),
          ),
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
        paint.color = Color(0xFF000000 | color);
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
