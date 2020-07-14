import React from 'react';
import logo from './logo.svg';

import './App.css';
import {Board} from "./Board"

export default class App extends React.Component {
    constructor(props) {
        super(props)
    }
    render() {
        import("./wasm_lib").then(module => {
            console.log(module.return_string())
        })
        return (
            <Board/>
        );
    }
}
