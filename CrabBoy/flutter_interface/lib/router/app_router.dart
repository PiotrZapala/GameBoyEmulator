import 'dart:typed_data';

import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_interface/pages/home_page.dart';
import 'package:flutter_interface/pages/games_page.dart';
import 'package:flutter_interface/pages/game_page.dart';

part 'app_router.gr.dart';

@MaterialAutoRouter(
  replaceInRouteName: 'Page,Route',
  routes: <AutoRoute>[
    AutoRoute(page: HomePage, initial: true),
    AutoRoute(page: GamesPage),
    AutoRoute(page: GamePage),
  ],
)
class AppRouter extends _$AppRouter {}
