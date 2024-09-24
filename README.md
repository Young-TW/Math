# Math

以 Rust 寫成的數學函式庫，提供了一些常用的數學函式。

預計的目錄結構如下：

```
Math/src/
│
├── Arithmetic_and_Basic_Math/  算術與基礎數學
│   ├── natural_numbers.rs  自然數
│   ├── integers.rs  整數
│   ├── rational_and_real_numbers.rs  有理數與實數
│   ├── basic_operations.rs  基本運算
│   ├── fractions_and_percentages.rs  分數與百分比
│   └── prime_numbers_and_factorization.rs  質數與因式分解
│
├── Algebra/  代數
│   ├── Elementary_Algebra/  初等代數
│   │   ├── equations.rs  方程式
│   │   ├── ratios.rs  比例
│   │   └── functions.rs  函數
│   ├── Advanced_Algebra/  高等代數
│   │   ├── polynomials.rs  多項式
│   │   ├── matrices.rs  矩陣
│   │   └── vectors.rs  向量
│   ├── Abstract_Algebra/  抽象代數
│   │   ├── group_theory.rs  群論
│   │   ├── ring_theory.rs  環論
│   │   └── field_theory.rs  體論
│   └── Linear_Algebra/  線性代數
│       ├── matrix_operations.rs  矩陣運算
│       ├── vector_spaces.rs  向量空間
│       └── eigenvalues_and_eigenvectors.rs  特徵值與特徵向量
│
├── Geometry/  幾何
│   ├── Euclidean_Geometry/  歐幾里得幾何
│   │   ├── points_lines_planes.rs  點、線、面
│   │   └── basic_shapes.rs  基本圖形
│   ├── Analytic_Geometry/  解析幾何
│   │   ├── coordinate_systems.rs  座標系統
│   │   ├── lines.rs  直線
│   │   ├── circles.rs  圓
│   │   └── conics.rs  圓錐曲線
│   ├── Spatial_Geometry/  空間幾何
│   │   ├── three_dimensional_shapes.rs  三維圖形
│   │   └── volume_and_surface_area.rs  體積與表面積
│   └── Differential_Geometry/  微分幾何
│       ├── curves.rs  曲線
│       └── surfaces.rs  曲面
│
├── Calculus/  微積分
│   ├── Differentiation/  微分
│   │   ├── derivatives.rs  導數
│   │   └── tangent_lines.rs  切線
│   ├── Integration/  積分
│   │   ├── definite_integrals.rs  定積分
│   │   └── indefinite_integrals.rs  不定積分
│   ├── Multivariable_Calculus/  多變數微積分
│   │   ├── partial_derivatives.rs  偏導數
│   │   └── multiple_integrals.rs  多重積分
│   └── Calculus_Applications/  微積分應用
│       ├── rates_of_change.rs  變化率
│       └── area_and_volume.rs  面積與體積
│
├── Statistics_and_Probability/  統計與機率
│   ├── Descriptive_Statistics/  敘述統計
│   │   ├── data_distributions.rs  數據分布
│   │   └── measures_of_central_tendency.rs  集中趨勢測量
│   ├── Probability_Theory/  機率論
│   │   ├── random_variables.rs  隨機變數
│   │   ├── probability_distributions.rs  機率分布
│   │   └── expectation_and_variance.rs  期望與變異數
│   └── Inferential_Statistics/  推論統計
│       ├── hypothesis_testing.rs  假設檢定
│       └── regression_analysis.rs  迴歸分析
│
├── Mathematical_Analysis/  數學分析
│   ├── Real_Analysis/  實變分析
│   │   ├── real_numbers_and_sequences.rs  實數與數列
│   │   └── limits_of_functions.rs  函數極限
│   ├── Complex_Analysis/  複變分析
│   │   ├── complex_numbers.rs  複數
│   │   └── residue_theorem.rs  留數定理
│   ├── Fourier_Analysis/  傅立葉分析
│   │   ├── fourier_series.rs  傅立葉級數
│   │   └── fourier_transforms.rs  傅立葉變換
│   └── Functional_Analysis/  泛函分析
│       ├── l2_spaces.rs  L2空間
│       └── hilbert_spaces.rs  希爾伯特空間
│
├── Discrete_Mathematics/  離散數學
│   ├── Combinatorics/  組合學
│   │   ├── permutations_and_combinations.rs  排列與組合
│   │   └── generating_functions.rs  生成函數
│   ├── Graph_Theory/  圖論
│   │   ├── graphs_and_trees.rs  圖與樹
│   │   └── paths_and_colorings.rs  路徑與染色
│   ├── Mathematical_Logic/  數理邏輯
│   │   ├── propositional_logic.rs  命題邏輯
│   │   └── predicate_logic.rs  謂詞邏輯
│   └── Automata_and_Computation/  自動機與計算理論
│       ├── finite_automata.rs  有限自動機
│       └── turing_machines.rs  圖靈機
│
├── Differential_Equations/  微分方程
│   ├── Ordinary_Differential_Equations/  常微分方程
│   │   ├── first_order_equations.rs  一階方程
│   │   └── second_order_equations.rs  二階方程
│   ├── Partial_Differential_Equations/  偏微分方程
│   │   ├── wave_equation.rs  波動方程
│   │   └── heat_equation.rs  熱傳導方程
│   └── Dynamical_Systems/  動力系統
│       ├── stability_theory.rs  穩定性理論
│       └── chaos_theory.rs  混沌理論
│
├── Number_Theory/  數論
│   ├── Elementary_Number_Theory/  初等數論
│   │   ├── primes_and_congruences.rs  質數與同餘
│   │   └── number_theory_functions.rs  數論函數
│   ├── Algebraic_Number_Theory/  代數數論
│   │   ├── number_fields.rs  數域
│   │   └── algebraic_integers.rs  代數整數
│   └── Analytic_Number_Theory/  解析數論
│       ├── prime_distribution.rs  質數分佈
│       └── dirichlet_series.rs  狄利克雷級數
│
├── Numerical_Analysis/  數值分析
│   ├── Numerical_Differentiation_and_Integration/  數值微分與積分
│   │   ├── numerical_differentiation.rs  數值微分
│   │   └── numerical_integration.rs  數值積分
│   ├── Numerical_Linear_Algebra/  數值線性代數
│   │   ├── solving_linear_systems.rs  線性系統求解
│   │   └── eigenvalue_problems.rs  特徵值問題
│   └── Numerical_Differential_Equations/  數值微分方程
│       ├── ode_methods.rs  常微分方程法
│       └── pde_methods.rs  偏微分方程法
│
├── Optimization/  最佳化
│   ├── Linear_Programming/  線性規劃
│   │   ├── simplex_method.rs  單純形法
│   │   └── duality_theory.rs  對偶理論
│   └── Nonlinear_Programming/  非線性規劃
│       ├── lagrange_multipliers.rs  拉格朗日乘數
│       └── convex_optimization.rs  凸優化
│
└── Topology/  拓撲學
    ├── Topological_Spaces/  拓撲空間
    │   ├── homeomorphisms.rs  同胚
    │   └── homotopy_theory.rs  同倫理論
    ├── Surfaces_and_Manifolds/  曲面與流形
    │   ├── surface_classification.rs  曲面分類
    │   └── manifolds.rs  流形
    └── Algebraic_Topology/  代數拓撲
        ├── fundamental_groups.rs  基本群
        └── homology_and_cohomology.rs  同調與上同調
```
