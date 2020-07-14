import React from 'react';
import logo from './logo.svg';

import './App.css';
import Counter from './Counter';

export default class App extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            counter: 0
        }
    }
    increment() {
        this.setState({
            counter: this.state.counter + 1
        })
    }
    render() {
        import("./wasm_lib").then(module => {
            console.log(module.return_string())
        })
        return (
            <div className="App">
                <header className="App-header">
                    <img src={logo} className="App-logo" alt="logo" />
                    <p>
                        Edit <code>src/App.js</code> and save to reload.
                </p>
                    <a
                        className="App-link"
                        href="https://reactjs.org"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        Learn React
                </a>
                </header>
                <Counter start={2} />
            </div>
        );
    }
}
