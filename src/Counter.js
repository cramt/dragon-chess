import React from 'react';

export default class Counter extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            counter: props.start || 0
        }
    }
    increment(){
        this.setState({
            count: this.state.counter + 1
        })
    }
    render() {
        return <p onClick={this.increment}>count: <b>{this.state.counter}</b></p>
    }
}