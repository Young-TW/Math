while 1:
    y = input ('')
    y = int(y)
    if y == None:
        break
    else:
        if (y%100!=0 and y%4==0) or y%400==0:
            print("閏年")
        else:
            print("平年")