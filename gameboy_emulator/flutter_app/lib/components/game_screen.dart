import 'package:flutter/material.dart';

class GameScreen extends StatelessWidget {
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
          painter: GameBoyScreenPainter(),
        ),
      ),
    );
  }
}

class GameBoyScreenPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    List<List<Color>> pixelData = generateSamplePixels();
    double pixelWidth = size.width / 160;
    double pixelHeight = size.height / 144;

    for (int y = 0; y < 144; y++) {
      for (int x = 0; x < 160; x++) {
        Paint paint = Paint()..color = pixelData[y][x];

        canvas.drawRect(
          Rect.fromLTWH(
              x * pixelWidth, y * pixelHeight, pixelWidth, pixelHeight),
          paint,
        );
      }
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }

  List<List<Color>> generateSamplePixels() {
    List<List<Color>> pixels = List.generate(
      144,
      (_) => List.generate(160, (_) => Colors.green),
    );

    for (int i = 0; i < 144; i++) {
      for (int j = 0; j < 160; j++) {
        pixels[i][j] = (i + j) % 2 == 0 ? Colors.black : Colors.white;
      }
    }
    return pixels;
  }
}
