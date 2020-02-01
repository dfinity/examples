import phonebook from 'ic:canisters/phonebook';
import * as React from 'react';
import { render } from 'react-dom';

class Phonebook extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
    };
  }

  async doInsert() {
    let name = document.getElementById("newEntryName").value;
    let desc = document.getElementById("newEntryDesc").value;
    let phone = document.getElementById("newEntryPhone").value;

    phonebook.insert(name, desc, parseInt(phone, 10));
  }

  async lookup() {
    let name = document.getElementById("lookupName").value;
    phonebook.lookup(name).then(opt_entry => {
      let [entry] = opt_entry;
      if (entry === null) {
        entry = { name: "", description: "", phone: ""};
      }
      document.getElementById("newEntryName").value = entry.name;
      document.getElementById("newEntryDesc").value = entry.description;
      document.getElementById("newEntryPhone").value = entry.phone.toString();
    });
  }

  render() {
    return (
      <div>
        <h1>PhoneBook</h1>
        <div>
          Insert or update new phonebook entry:
          <table>
            <tr><td>Name:</td><td><input id="newEntryName"></input></td></tr>
            <tr><td>Description:</td><td><input id="newEntryDesc"></input></td></tr>
            <tr><td>Phone:</td><td><input id="newEntryPhone" type="number"></input></td></tr>
          </table>
          <button onClick={() => this.doInsert()}>Insert or Update</button>
        </div> 
        <div>
          Lookup Name: <input id="lookupName"></input> <button onClick={
            () => this.lookup()
          }>Lookup</button>
        </div>
      </div>
    );
  }
}

document.title = "DFINITY PHONEBOOK EXAMPLE";

render(<Phonebook />, document.getElementById('app'));
