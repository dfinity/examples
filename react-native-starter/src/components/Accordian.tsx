import React from "react";
import {
  View,
  TouchableOpacity,
  Text,
  StyleSheet,
  LayoutAnimation,
} from "react-native";

interface accordianProps {
  title: string;
  data: any;
}
const Accordian = ({ data, title }: accordianProps) => {
  const [expanded, setExpand] = React.useState(false);

  const toggleExpand = () => {
    LayoutAnimation.configureNext(LayoutAnimation.Presets.easeInEaseOut);
    setExpand((expandedState) => !expandedState);
  };

  return (
    <View>
      <TouchableOpacity style={styles.row} onPress={() => toggleExpand()}>
        <Text style={[styles.title]}>{title}</Text>
        <Text>{expanded ? "-" : "+"}</Text>
      </TouchableOpacity>
      <View style={styles.parentHr} />
      {expanded && (
        <View style={styles.child}>
          <Text>{data}</Text>
        </View>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  title: {
    fontSize: 14,
  },
  row: {
    flexDirection: "row",
    justifyContent: "space-between",
    height: 56,
    paddingLeft: 25,
    paddingRight: 18,
    alignItems: "center",
    backgroundColor: "lightgrey",
  },
  parentHr: {
    height: 1,
    color: "white",
    width: "100%",
  },
  child: {
    backgroundColor: "lightgrey",
    padding: 16,
  },
});

export default Accordian;
