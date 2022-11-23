import React from "react";
import { View, Text, TextInput, StyleSheet } from "react-native";

const Input = ({
  placeholder,
  onChange,
  title,
}: {
  placeholder: string;
  onChange: (text: string) => void;
  title: string;
}) => {
  return (
    <View style={styles.container}>
      <Text>{title}</Text>
      <View style={styles.inputBox}>
        <TextInput style={styles.input} placeholder={placeholder} onChangeText={onChange} />
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    padding: 18,
  },
  inputBox: {
    borderWidth: 0.5,
    borderRadius: 6,
    color: "grey",
    borderColor: "grey",
    height: 55,
    display: "flex",
    justifyContent: "center",
    marginTop: 6,
    marginBottom: 2,
  },
  input: {
    marginLeft: 10,
  },
});

export default Input;
