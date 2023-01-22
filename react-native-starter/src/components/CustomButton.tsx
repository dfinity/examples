import React from "react";
import {
  TouchableOpacity,
  Text,
  View,
  StyleSheet,
  GestureResponderEvent,
} from "react-native";

interface customButtonInput {
  title: string;
  onPress: (event: GestureResponderEvent) => void;
  width?: number;
  color?: string;
  fontSize?: number;
}

function CustomButton({
  title,
  onPress,
  width,
  color,
  fontSize,
}: customButtonInput) {
  return (
    <TouchableOpacity onPress={onPress}>
      <View
        style={[
          styles.buttonContainer,
          { width: width, backgroundColor: color },
        ]}
      >
        <Text style={[styles.title, { fontSize: fontSize }]}>{title}</Text>
      </View>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  buttonContainer: {
    backgroundColor: "blue",
    height: 50,
    borderRadius: 16,
    margin: 10,
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    alignSelf: "center",
  },
  title: {
    color: "white",
    fontSize: 18,
  },
});

export default CustomButton;
