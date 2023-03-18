import React, { useEffect, useState } from 'react';
import { StyleSheet, Text, View, Image, TouchableOpacity, BackHandler } from 'react-native';
import { baseUrl } from './config';


function useBackButton(handler) {
  useEffect(() => {
    BackHandler.addEventListener("hardwareBackPress", handler);

    return () => {
      BackHandler.removeEventListener(
        "hardwareBackPress",
        handler
      );
    };
  }, [handler]);
}

const hello = async () => {
  const response = await fetch(`${baseUrl}/hello/bla`);
  const t = await response.text();
};

export default function App() {
  const [isWelcome, setIsWelcome] = useState(true);

  const handlePress = () => {
    setIsWelcome(false);
  };

  useBackButton(() => {
    if (!isWelcome) {
      setIsWelcome(true)
      return true;
    }
    // default handler
    return false;
  });

  if (isWelcome) {
    return (
      <View style={styles.container}>
        <Text style={styles.welcomeText}>Sinder</Text>
        <TouchableOpacity onPress={handlePress} style={styles.button}>
          <Text style={styles.buttonText}>...</Text>
        </TouchableOpacity>
        <TouchableOpacity onPress={hello} style={styles.button}>
          <Text style={styles.buttonText}>Hello</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Text style={styles.text}>First line of text</Text>
      <Text style={styles.text}>Second line of text 2</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#e00',
    alignItems: 'center',
    justifyContent: 'center',
    display:'flex',
  },
  welcomeText: {
    fontSize: 24,
    marginTop: 20,
  },
  button: {
    backgroundColor: '#0088cc',
    padding: 10,
    borderRadius: 5,
    marginTop: 20,
  },
  buttonText: {
    color: '#fff',
    fontSize: 18,
    position: 'relative'
    
  },
  text: {
    fontSize: 24,
    marginVertical: 10,
  },
});
