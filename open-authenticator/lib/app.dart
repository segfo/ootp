import 'package:flutter/cupertino.dart';

class Application extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return CupertinoApp(
      theme: CupertinoThemeData(
        primaryColor: CupertinoColors.systemBlue,
        barBackgroundColor: CupertinoDynamicColor.withBrightness(
            color: CupertinoColors.white,
            darkColor: CupertinoColors.black,
        ),
        scaffoldBackgroundColor: CupertinoDynamicColor.withBrightness(
            color: CupertinoColors.white.withAlpha(240),
            darkColor: CupertinoColors.black
        ),
      ),
      home: HomeScreen(),
    );
  }
}

class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      child: ListView(
        children: [
          Container(
            margin: EdgeInsets.symmetric(
              horizontal: 12.0,
              vertical: 6.0
            ),
            padding: EdgeInsets.symmetric(
              horizontal: 12.0,
              vertical: 8.0,
            ),
            decoration: BoxDecoration(
              color: CupertinoDynamicColor.withBrightness(
                color: CupertinoColors.white,
                darkColor: CupertinoColors.white.withAlpha(30),
              ).resolveFrom(context),
              borderRadius: BorderRadius.circular(16),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text("GitHub", style: CupertinoTheme.of(context).textTheme.navTitleTextStyle,),
                Text("medz", style: CupertinoTheme.of(context).textTheme.textStyle),
                Text("123 456")
              ],
            ),
          ),
          CupertinoButton(
            child: Text("Do you want to setup Authenticator or get help?", style: TextStyle(
              fontSize: 12.0,
            ),),
            onPressed: (){},
          ),
        ],
      ),
      navigationBar: CupertinoNavigationBar(
        backgroundColor: CupertinoTheme.of(context).scaffoldBackgroundColor,
        middle: CupertinoSearchTextField(),
        trailing: CupertinoButton(
          padding: EdgeInsets.zero,
          child: Icon(CupertinoIcons.add),
          onPressed: () {},
        ),
      ),
    );
  }
}