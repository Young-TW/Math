#include <iostream>
using namespace std;

int main()
{
	int n1,n2,c;
	cout << "請輸入兩個正整數:";
	cin >> n1 >> n2;
	
	if (n1>n2){
		c=n1;
	}
	else{
		c=n2;
	}
	
	do
	{
		if (c%n1==0 && c%n2==0)
		{
			cout << "最小公倍數為:" << c;
			break; 
		}
		else
			c++;			
	}while (true);
	cout << endl;
	
	return 0;
}