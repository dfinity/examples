/* eslint-disable prettier/prettier */
/* eslint-disable react-native/no-inline-styles */
/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * Generated with the TypeScript template
 * https://github.com/react-native-community/react-native-template-typescript
 *
 * @format
 */
import React, { useEffect } from "react";
import {
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  View,
  Image,
  Linking,
} from "react-native";

import { greet_dapp } from "./declarations/greet_dapp";
import Input from "./components/Input";
import CustomButton from "./components/CustomButton";
import "react-native-polyfill-globals/auto";
import Accordian from "./components/Accordian";

const App = () => {
  const [showText, setShowText] = React.useState(true);
  const [value, setValue] = React.useState("");
  const [result, setResult] = React.useState("I");
  const [blink, setBlink] = React.useState(true);

  const greetName = async (input: string) => {
    setResult("I");
    setBlink(true);
    //Interact with foo actor, calling the greet method
    try {
      const greeting = await greet_dapp.greet(input);
      setResult(greeting);
      setBlink(false);
      setShowText(true);
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    // blink effect
    const interval = setInterval(() => {
      setShowText((text: Boolean) => !text);
    }, 300);
    if (!blink) {
      return clearInterval(interval);
    }
    return () => clearInterval(interval);
  }, [blink]);

  const aboutRN = {
    title: "A React Native Starter",
    data: "React Native uses React and Javascript to build native components for mobile devices. \n\nThis React Native app makes calls to a deployed backend canister on the IC.",
  }
  
  return (
    <SafeAreaView style={{flex: 1}}>
      <StatusBar barStyle="dark-content" backgroundColor={'beige'} />
      <ScrollView contentContainerStyle={{ flexGrow: 1 }}>
        <View style={styles.container}>
          <View style={{ padding: 20 }}>
            <Accordian title={aboutRN.title} data={aboutRN.data} />
        </View>
          <View style={styles.header}>
            <Text style={styles.headerText}>Greet App</Text>
            
          </View>
          <View style={styles.greetForm}>
            <Input color='#dfe3f5'
              placeholder={'Enter your name here'}
              onChange={text => setValue(text)}
              title={'Enter your Name'}
            />
            <CustomButton
              width={180} color='#4b68c9'
              title="Submit"
              onPress={e => {
                e.preventDefault();
                greetName(value);
              }}
            />
          </View>
          <View style={{padding: 10}}>
            <Text
              style={{
                fontSize: 14,
                paddingLeft: 20,
                fontWeight: 'bold',
                marginTop: 15,
              }}>
              Response
            </Text>
            <View style={styles.sectionResponse}>
              <Text
                style={{
                  textAlign: 'center',
                  color: 'yellow',
                  display: showText ? 'flex' : 'none',
                }}>
                {result}
              </Text>
            </View>
          </View>
          <View style={styles.bottom}>
          <Image
              source={require('./assets/icon-192x192.png')}
              style={{width: 60, height: 60, margin: 'auto'}}
            />
            <Text
              style={{
                fontSize: 14,
                marginBottom: 10,
                textAlign: 'center',
              }}>
              Interested to explore the Internet Computer?
            </Text>
            <CustomButton
              width={120} color='#634cb5' fontSize={14}
              title="Learn more"
              onPress={e => {
                e.preventDefault();
                Linking.openURL("https://internetcomputer.org/");
              }}
            />
          </View>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flexGrow: 1,
  },
  header: {
    height: 'auto',
    padding: 28,
  },
  headerText: {
    textAlign: 'center',
    fontSize: 28,
    fontWeight: 'bold'
  },
  greetForm: {
    backgroundColor: '#cad1ed',
    borderRadius: 20,
    padding: 20,
    paddingTop: 30,
    margin: 20,
  },
  sectionContainer: {
    marginTop: 32,
    paddingHorizontal: 24,
  },
  sectionTitle: {
    fontSize: 24,
    fontWeight: '600',
    color: 'white',
  },
  sectionDescription: {
    marginTop: 8,
    fontSize: 18,
    fontWeight: '400',
    color: 'black',
  },
  highlight: {
    fontWeight: '700',
  },
  sectionResponse: {
    marginTop: 10,
    marginHorizontal: 20,
    minHeight: 70,
    display: 'flex',
    textAlign: 'center',
    justifyContent: 'center',
    backgroundColor: 'black',
    borderRadius: 10,
  },
  bottom: {
    flexGrow: 2,
    display: 'flex',
    justifyContent: 'flex-end',
    paddingBottom: 10,
    alignItems: 'center',
  }
});

export default App;
