import React from 'react';
import "./Board.css"

export class Board extends React.Component {
    constructor(props) {
        super(props)
    }
    render() {
        let i = 0;
        return (
            <div>
                {["lower", "middle", "upper"].map(x => {
                    let classNames = ["white", "black"].map(y => y + "-" + x)
                    return <div className="chessboard">
                        {Array.from(Array(8).keys()).map(y =>
                            Array.from(Array(12).keys()).map(z => <div className={classNames[Number(((Number(y % 2 == 0) + z) % 2 == 0))]}></div>))}
                    </div>
                })}
            </div>

        )
    }
}