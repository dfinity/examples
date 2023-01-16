/**
 * @format
 */
import "react-native-polyfill-globals/auto";
import "react-native-fetch-api";
import "fast-text-encoding";
import { AppRegistry } from "react-native";
import App from "./src/App.tsx";
import { name as appName } from "./app.json";

AppRegistry.registerComponent(appName, () => App);
