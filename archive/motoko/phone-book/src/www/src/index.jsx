import * as React from "react";
import { render } from "react-dom";
import { phone_book as canister } from "../../declarations/phone_book";

class PhoneBook extends React.Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  async doInsert() {
    let name = document.getElementById("newEntryName").value;
    let desc = document.getElementById("newEntryDesc").value;
    let phone = document.getElementById("newEntryPhone").value;
    canister.insert(name, { desc, phone });
  }

  async lookup() {
    let name = document.getElementById("lookupName").value;
    canister.lookup(name).then((opt_entry) => {
      let entry = opt_entry.length > 0 ? opt_entry[0] : null;
      if (entry === null || entry === undefined) {
        entry = {
          desc: "",
          phone: "",
        };
      }
      document.getElementById("newEntryName").value = name;
      document.getElementById("newEntryDesc").value = entry.desc;
      document.getElementById("newEntryPhone").value = entry.phone;
    });
  }

  render() {
    return (
      <div>
        <h1>Phone Book</h1>
        <div>
          Insert or update a new phone book entry:
          <table>
            <tr>
              <td>Name:</td>
              <td>
                <input required id="newEntryName"></input>
              </td>
            </tr>
            <tr>
              <td>Description:</td>
              <td>
                <input id="newEntryDesc"></input>
              </td>
            </tr>
            <tr>
              <td>Phone:</td>
              <td>
                <input
                  required
                  id="newEntryPhone"
                  type="tel"
                  pattern="[0-9]{10}"
                ></input>
              </td>
            </tr>
          </table>
          <button onClick={() => this.doInsert()}>Insert or Update</button>
        </div>
        <div>
          Lookup Name: <input id="lookupName"></input>{" "}
          <button onClick={() => this.lookup()}>Lookup</button>
        </div>
      </div>
    );
  }
}

render(<PhoneBook />, document.getElementById("app"));
