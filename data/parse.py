import csv
import os
import sys


def handleRow(row):
    cells = [cell.strip() for cell in row.split(",")]
    if len(cells) != 4: 
        return False, cells
    
    pairA, pts, pairB, pts2 = cells
    
    a,b = pairA.split('/')
    x,y = pairB.split('/')
    ptsAB = pts
    ptsXY = pts2
    return True, [a,b,ptsAB, x,y,ptsXY]
    

def convertFile(file, newHeaders):
    with open(file, "r") as f:
        lines = [l.strip() for l in f.readlines()]
        if len(lines) < 2:
            raise ValueError("File is missing header or date line")
        
        date = lines[0]
        rows = lines[2::]
        newData = [newHeaders]
        goodRows = []
        badRows = []
        for row in rows:
            result, newRow = handleRow(row)
            if (result):
                goodRows.append(newRow) 
            else:
                badRows.append(newRow)

        
        if len(badRows):
            print(f"encountered bad rows: {file}", file=sys.stderr)
            [print(row, file=sys.stderr) for row in badRows]
        
        print(",".join(newHeaders))
        [print(",".join(([date, str(i+1)] + row))) for i, row in enumerate(goodRows)]

def main(args):
    headers= "Date","GameNo","A","B","PtsAB","X","Y","PtsXY"
    convertFile(args[0], headers)
    

if __name__== "__main__":
    main(sys.argv[1:])
    