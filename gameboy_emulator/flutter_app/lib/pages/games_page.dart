import 'dart:io';
import 'dart:typed_data';
import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter_app/router/app_router.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class GamesPage extends StatefulWidget {
  @override
  _GamesPageState createState() => _GamesPageState();
}

class _GamesPageState extends State<GamesPage> {
  List<String> _gameFiles = [];

  @override
  void initState() {
    super.initState();
    _loadGameFiles();
  }

  Future<void> _loadGameFiles() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    List<String>? storedFiles = prefs.getStringList('gameFiles');
    if (storedFiles != null) {
      setState(() {
        _gameFiles = storedFiles;
      });
    }
  }

  Future<void> _saveGameFiles() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setStringList('gameFiles', _gameFiles);
  }

  Future<String> _getRomStoragePath() async {
    final directory = await getApplicationDocumentsDirectory();
    return directory.path;
  }

  Future<void> _pickAndSaveGameFile() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles(
      type: FileType.any,
    );

    if (result != null) {
      String? filePath = result.files.single.path;
      if (filePath != null && !_gameFiles.contains(filePath)) {
        String savedFilePath = await _saveRomToInternalStorage(filePath);
        setState(() {
          _gameFiles.add(savedFilePath);
        });
        _saveGameFiles();
      }
    } else {
      print('No file selected');
    }
  }

  Future<String> _saveRomToInternalStorage(String filePath) async {
    final storagePath = await _getRomStoragePath();
    final fileName = filePath.split('/').last;
    final newFilePath = '$storagePath/$fileName';
    await File(filePath).copy(newFilePath);
    return newFilePath;
  }

  Future<void> _openGame(String gamePath) async {
    try {
      File romFile = File(gamePath);
      Uint8List romData = await romFile.readAsBytes();
      String gameName = _formatGameName(gamePath);

      Uint8List? ramData = await _loadGameRam(gameName);

      context.router.push(
          GameRoute(romData: romData, gameName: gameName, ramData: ramData));
    } catch (e) {
      print("Błąd podczas odczytu ROM: $e");
    }
  }

  Future<Uint8List?> _loadGameRam(String gameName) async {
    final storagePath = await _getRomStoragePath();
    final ramFilePath = '$storagePath/$gameName.sav';

    if (await File(ramFilePath).exists()) {
      return await File(ramFilePath).readAsBytes();
    }
    return null;
  }

  String _formatGameName(String filePath) {
    String fileName = filePath.split('/').last;
    if (fileName.endsWith('.gb')) {
      fileName = fileName.replaceAll('.gb', '');
    }
    return fileName.toUpperCase();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        iconTheme: IconThemeData(color: Colors.white),
        automaticallyImplyLeading: true,
        centerTitle: true,
        title: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.videogame_asset, size: 36, color: Colors.white),
            SizedBox(width: 10),
            Text(
              'Games',
              style: TextStyle(
                fontSize: 30,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
          ],
        ),
      ),
      body: Stack(
        children: [
          Positioned.fill(
            child: Image.asset(
              'assets/backgrounds/app_background.png',
              fit: BoxFit.cover,
            ),
          ),
          Positioned.fill(
            child: Column(
              children: [
                SizedBox(height: 5),
                Expanded(
                  child: _gameFiles.isEmpty
                      ? Center(
                          child: Text(
                            'No games added',
                            style: TextStyle(
                              fontSize: 20,
                              color: Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        )
                      : ListView.builder(
                          itemCount: _gameFiles.length,
                          itemBuilder: (context, index) {
                            return ListTile(
                              visualDensity: VisualDensity(vertical: -6),
                              contentPadding:
                                  EdgeInsets.symmetric(horizontal: 16.0),
                              leading: Icon(Icons.videogame_asset,
                                  color: Colors.white),
                              title: Text(
                                _formatGameName(_gameFiles[index]),
                                style: TextStyle(
                                  color: Colors.white,
                                  fontSize: 25,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              onTap: () => _openGame(_gameFiles[index]),
                            );
                          },
                        ),
                ),
              ],
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _pickAndSaveGameFile,
        child: Icon(Icons.add),
      ),
    );
  }
}
