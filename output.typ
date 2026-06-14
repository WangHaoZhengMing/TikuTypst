#let forrender = (
  title([一、填空题]),
  fb(
    [#latex(`\dfrac{6}{5} =`) #blank_line() #latex(`\div 10 = 18 :`) #blank_line() #latex(`= \dfrac{6 + \text{ (<span style="white-space: pre-wrap">  </span>) }}{5 + 10} =`) #blank_line() #latex(`\%`) #latex(`=`) #blank_line() (填小数)．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`12`)], [#latex(`15`)], [#latex(`120`)], [#latex(`1.2`)]),
    analysis: [解：根据分数、除法与比的关系，以及分数的基本性质和数之间的互化进行解答．

      由分数的分子、分母和除法中的被除数、除数的关系可得：#latex(`\dfrac{6}{5} = 6 \div 5 = (6 \times 2) \div (5 \times 2) = 12 \div 10`)；

      根据比与分数的关系可得：#latex(`\dfrac{6}{5} = 6 : 5 = (6 \times 3) : (5 \times 3) = 18 : 15`)；

      根据分数的基本性质，分母由 #latex(`5`) 变为 #latex(`5 + 10 = 15`)，扩大了 #latex(`3`) 倍，分子也应扩大 #latex(`3`) 倍变为 #latex(`18`)，即 #latex(`6 + (12) = 18`)；

      将分数化为百分数和小数：#latex(`\dfrac{6}{5} = 1.2 = 120\%`)．

      所以，#latex(`\dfrac{6}{5} = 12 \div 10 = 18 : 15 = \dfrac{6 + (12)}{5 + 10} = 120\% = 1.2`)．

      故答案为：#latex(`12`)，#latex(`15`)，#latex(`120`)，#latex(`1.2`)．],
  ),
  fb(
    [如果篮球比赛输一场记作 #latex(`-1`) 分，那么输两场记作 #blank_line() 分，#latex(`+3`) 分表示 #blank_line()．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`-2`)], [赢三场]),
    analysis: [解：因为篮球比赛输一场记作 #latex(`-1`) 分，

      所以输两场应记作 #latex(`-2`) 分；

      因为 “#latex(`+`)” 和 “#latex(`-`)” 表示具有相反意义的量，输球记为负，则赢球记为正，

      所以 #latex(`+3`) 分表示赢三场．

      故答案为：#latex(`-2`)；赢三场．],
  ),
  fb(
    [分数#latex(`\dfrac{a}{5}`)，它的分数单位是#blank_line() ，当#latex(`a=`)#blank_line() 时，它是最大的真分数；当#latex(`a=`)#blank_line() 时，它是最小的合数．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`\dfrac{1}{5}`)], [#latex(`4`)], [#latex(`20`)]),
    analysis: [本题考查了分数的相关概念，解答本题要明确分数单位的定义，真分数的定义，合数的定义．欲求这个分数的分数单位，可根据分数单位的定义求解；欲求当a等于几时，它是最大的真分数，可根据真分数的定义求解；欲求当a等于几时，它是最小合数，可根据最小的合数是#latex(`4`)求解．

      解：#latex(`\dfrac{a}{5}`)的分数单位是#latex(`\dfrac{1}{5}`)，根据真分数的定义，当#latex(`a=4`)时，它是最大的真分数，根据合数的定义，当#latex(`a=20`)时，#latex(`\dfrac{a}{5}`)是最小的合数#latex(`4`)．

      故答案为：#latex(`\dfrac{1}{5}`)；#latex(`4`)；#latex(`20`)],
  ),
  fb(
    [如果 #latex(`4 \square 6`) 是 #latex(`4`) 的倍数，#latex(`\square`) 里最大应填 #blank_line()；如果 #latex(`79 \square`) 是 #latex(`6`) 的倍数，#latex(`\square`) 里可以填 #blank_line()．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`9`)], [#latex(`2`)或#latex(`8`)]),
    analysis: [解：根据能被 #latex(`2`)，#latex(`3`)，#latex(`4`) 整除的数的特征：

      第一空：要使三位数 #latex(`4 \square 6`) 是 #latex(`4`) 的倍数，只需其末两位数 #latex(`\square 6`) 是 #latex(`4`) 的倍数．

      由于 #latex(`\square 6`) 是 #latex(`4`) 的倍数，#latex(`\square`) 中可以填 #latex(`1`)，#latex(`3`)，#latex(`5`)，#latex(`7`)，#latex(`9`)，其中最大的是 #latex(`9`)，所以 #latex(`\square`) 里最大应填 #latex(`9`)；

      第二空：要使三位数 #latex(`79 \square`) 是 #latex(`6`) 的倍数，它必须同时是 #latex(`2`) 和 #latex(`3`) 的倍数．

      因为是 #latex(`2`) 的倍数，所以个位 #latex(`\square`) 必须是偶数，即 #latex(`0`)，#latex(`2`)，#latex(`4`)，#latex(`6`)，#latex(`8`)；

      因为是 #latex(`3`) 的倍数，所以各位数字之和 #latex(`7 + 9 + \square = 16 + \square`) 必须是 #latex(`3`) 的倍数．

      当 #latex(`\square = 2`) 时，#latex(`16 + 2 = 18`) 是 #latex(`3`) 的倍数；

      当 #latex(`\square = 8`) 时，#latex(`16 + 8 = 24`) 是 #latex(`3`) 的倍数．

      所以 #latex(`\square`) 里可以填 #latex(`2`) 或 #latex(`8`)．

      故答案为：#latex(`9`)，#latex(`2`)或#latex(`8`)．],
  ),
  fb(
    [随意找 #latex(`13`) 个小朋友，他们中至少有 #blank_line() 个人的属相相同．这是我们学习的“鸽巢问题”，题中 #blank_line() 相当于“鸽子”．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`2`)], [#latex(`13`)个小朋友]),
    analysis: [解：根据鸽巢原理，一共有 #latex(`12`) 个属相，即有 #latex(`12`) 个“鸽巢”；

      有 #latex(`13`) 个小朋友，即有 #latex(`13`) 只“鸽子”．

      #latex(`13 \div 12 = 1`) （个） #latex(`1`) （个），

      #latex(`1 + 1 = 2`) （个），

      所以他们中至少有 #latex(`2`) 个人的属相相同；

      题中的“#latex(`13`)个小朋友”相当于“鸽子”．

      故答案为：#latex(`2`)；#latex(`13`)个小朋友．],
  ),
  composite(
    [按规律填数．],
    (
      fb(
        [#latex(`1`)， #latex(`4`)， #latex(`9`)， #latex(`16`)， #blank_line()， #blank_line() #latex(`...`)],
        difficulty: 2,
        business-type: "CSX-ZHT",
        answer: ([#latex(`25`)], [#latex(`36`)]),
        analysis: [解：通过观察已知的数列：

          第一个数：#latex(`1 = 1^2`)

          第二个数：#latex(`4 = 2^2`)

          第三个数：#latex(`9 = 3^2`)

          第四个数：#latex(`16 = 4^2`)

          由此可得规律：第 #latex(`n`) 个数是 #latex(`n^2`)．

          因此，第五个数为 #latex(`5^2 = 25`)，第六个数为 #latex(`6^2 = 36`)．

          故答案为：#latex(`25`)，#latex(`36`)．],
      ),
      fb(
        [#latex(`1`)， #latex(`2`)， #latex(`2`)， #latex(`4`)， #latex(`3`)， #latex(`8`)， #latex(`4`)， #latex(`16`)， #latex(`5`)， #blank_line()， #blank_line() #latex(`...`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`32`)], [#latex(`6`)]),
        analysis: [解：根据题意，观察数列：#latex(`1`)，#latex(`2`)，#latex(`2`)，#latex(`4`)，#latex(`3`)，#latex(`8`)，#latex(`4`)，#latex(`16`)，#latex(`5`)，……

          可以发现该数列由两个子数列交替排列组成：

          奇数项依次为：#latex(`1`)，#latex(`2`)，#latex(`3`)，#latex(`4`)，#latex(`5`)，……，是公差为 #latex(`1`) 的等差数列；

          偶数项依次为：#latex(`2`)，#latex(`4`)，#latex(`8`)，#latex(`16`)，……，是公比为 #latex(`2`) 的等比数列．

          因此，第 #latex(`10`) 项为偶数项的第 #latex(`5`) 项，应为 #latex(`16 \times 2 = 32`)；

          第 #latex(`11`) 项为奇数项的第 #latex(`6`) 项，应为 #latex(`5 + 1 = 6`)．

          故答案为：#latex(`32`)，#latex(`6`)．],
      ),
    ),
    difficulty: 3,
    business-type: "CSX-ZHT",
  ),
  fb(
    [#latex(`A = 2 \times 2 \times 3 \times 5`)，#latex(`B = 2 \times 3 \times 3 \times 5`)，#latex(`A`)和#latex(`B`)的最大公因数是 #blank_line()，#latex(`A`)和#latex(`B`)的最小公倍数是 #blank_line()．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`30`)], [#latex(`180`)]),
    analysis: [解：因为 #latex(`A = 2 \times 2 \times 3 \times 5`)，

      #latex(`B = 2 \times 3 \times 3 \times 5`)．

      所以 #latex(`A`) 和 #latex(`B`) 的公有质因数是 #latex(`2`)，#latex(`3`)，#latex(`5`)．

      最大公因数是这些公有质因数的乘积：

      #latex(`2 \times 3 \times 5 = 30`)；

      最小公倍数是公有质因数与独有质因数的乘积：

      #latex(`2 \times 2 \times 3 \times 3 \times 5 = 180`)．

      故答案为：#latex(`30`)；#latex(`180`)．],
  ),
  fb(
    [如图，一张方桌能坐#latex(`4`)人，#latex(`2`)张方桌拼在一起能坐#latex(`6`)人．．．．．．，#latex(`10`)张方桌拼在一起能坐#blank_line() 人；要坐#latex(`32`)人，需要#blank_line() 张方桌拼在一起；n张方桌拼在一起能坐#blank_line() 人．

      #box(baseline: 25%, image(
        "https://k12static.xdf.cn/k12/xkw/1749864779591/867612c5-0512-47af-a660-408c6e0a5bec.png",
        width: 35.93%,
      ))],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`22`)], [#latex(`15`)], [#latex(`2n+2`)]),
    analysis: [本题主要考查了数与形中的找规律问题，根据#latex(`1`)张方桌能坐：#latex(`2+2=4`)，#latex(`2`)张方桌能坐：#latex(`2\times 2+2=6`)人，#latex(`3`)张方桌能坐：#latex(`2\times 3+2=8`)人，即可得出#latex(`10`)张方桌能坐#latex(`2\times 10+2=22`)，用#latex(`\left(32-2\right)\div 2=15`)，即可得出要坐#latex(`32`)人需要#latex(`\left(32-2\right)\div 2=15`)张桌子，总结规律即n张方桌拼在一起能坐#latex(`2n+2`)人．

      解：#latex(`1`)张方桌能坐：#latex(`2+2=4`)（人），

      #latex(`2`)张方桌能坐：#latex(`2\times 2+2=6`)（人），

      #latex(`3`)张方桌能坐：#latex(`2\times 3+2=8`)（人），

      #latex(`\cdot \cdot \cdot`)

      #latex(`10`)张方桌能坐：#latex(`2\times 10+2=22`)（人），

      要坐#latex(`32`)人，需要：#latex(`\left(32-2\right)\div 2=15`)（张），

      n张方桌拼在一起能坐#latex(`2n+2`)人，

      故答案为：#latex(`22`)，#latex(`15`)，#latex(`2n+2`)．],
  ),
  fb(
    [把 #latex(`0.333`)，#latex(`\dfrac{1}{3}`)，#latex(`33\%`)，#latex(`0.34`)，#latex(`0.4`)，三成五从左到右依次从大到小排列，排在第四位的数是 (#blank_line())．],
    difficulty: 3,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`\dfrac{1}{3}`)],),
    analysis: [解：为了方便比较，先把这些数统一化成小数：

      #latex(`0.333`)；

      #latex(`\dfrac{1}{3} \approx 0.3333`)；

      #latex(`33\% = 0.33`)；

      #latex(`0.34`)；

      #latex(`0.4`)；

      三成五 #latex(`= 35\% = 0.35`)．

      因为 #latex(`0.4 > 0.35 > 0.34 > 0.3333 > 0.333 > 0.33`)，

      所以它们从大到小的排列顺序为：

      #latex(`0.4 > \text{三成五} > 0.34 > \dfrac{1}{3} > 0.333 > 33\%`)．

      其中排在第四位的数是 #latex(`\dfrac{1}{3}`)．

      故答案为：#latex(`\dfrac{1}{3}`)．],
  ),
  fb(
    [当 #latex(`x = 0.5`) 时，#latex(`x^2 =`) #blank_line()，#latex(`2x =`) #blank_line()；当 #latex(`y =`) #blank_line() 时，#latex(`y^2 = 2y`)．],
    difficulty: 2,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`0.25`)], [#latex(`1`)], [#latex(`0`) 或 #latex(`2`)]),
    analysis: [解：当 #latex(`x = 0.5`) 时，

      #latex(`x^2 = 0.5^2 = 0.25`)；

      #latex(`2x = 2 \times 0.5 = 1`)．

      当 #latex(`y^2 = 2y`) 时，

      #latex(`y^2 - 2y = 0`)，

      #latex(`y(y - 2) = 0`)，

      解得 #latex(`y = 0`) 或 #latex(`y = 2`)．

      所以当 #latex(`y = 0`) 或 #latex(`2`) 时，#latex(`y^2 = 2y`)．

      故答案为：#latex(`0.25`)；#latex(`1`)；#latex(`0`) 或 #latex(`2`)．],
  ),
  subj(
    [一个圆柱的底面半径是#latex(`3\mathrm{cm}`)，它的侧面展开图正好是一个正方形，与这个圆柱等底等高的圆锥的体积是多少．],
    difficulty: 2,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`177.4728\mathrm{cm}^{3}`)],),
    analysis: [根据圆柱侧面展开图正好是一个正方形，求出高，再根据圆锥体积公式计算即可．

      解：圆柱底面周长为：#latex(`3\times 2\times 3.14=18.84\left(\mathrm{cm}\right)`)，

      由于侧面展开图正好是一个正方形，

      则高为#latex(`18.84\mathrm{cm}`)，

      则等底等高的圆锥的体积是#latex(`3^{2}\times 3.14\times 18.84\times \dfrac{1}{3}=177.4728\mathrm{cm}^{3}`)．

      本题考查了圆柱的侧面展开图，圆柱和圆锥的关系，圆锥的体积，解题的关键是根据侧面展开图求出高．],
  ),
  fb(
    [(如下图) #latex(`5`)个棱长#latex(`2`)分米的正方体硬纸箱堆放在墙角，体积一共是 #blank_line() 立方分米，露在外面的硬纸面积是 #blank_line() 平方分米．

      #box(baseline: 25%, image(
        "https://img.xkw.com/dksih/QBM/2020/2/26/2407331122192384/2411489676763137/STEM/d5fd875450724b67946bbea0f3474f8a.png",
        width: 20%,
      ))],
    difficulty: 3,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`40`)], [#latex(`40`)]),
    analysis: [解：每个正方体的体积为：#latex(`2 \times 2 \times 2 = 8`)（立方分米），

      #latex(`5`) 个正方体的总体积为：#latex(`8 \times 5 = 40`)（立方分米）．

      根据正方体堆放在墙角的方式，可以通过从不同方向观察来确定露在外面的面数：

      无论是按照下层 #latex(`4`) 个、上层 #latex(`1`) 个的方式，还是下层 #latex(`3`) 个、上层 #latex(`2`) 个的方式靠墙角堆放，露在外面的面数总共都是 #latex(`10`) 个．

      每个面的面积为：#latex(`2 \times 2 = 4`)（平方分米），

      所以露在外面的硬纸面积是：#latex(`10 \times 4 = 40`)（平方分米）．

      故答案为：#latex(`40`)，#latex(`40`)．],
  ),
  title([二、判断题]),
  fb(
    [一根绳子长#latex(`\dfrac{4}{5}`)米，也就是#latex(`80%`)米．#blank_line()],
    difficulty: 1,
    business-type: "CYW-PDT",
    answer: ([×],),
    analysis: [根据百分数的意义：表示一个数是另一数的百分之几的数叫百分数，百分数又叫百分率或百分比；它只能表示两数之间的倍数关系，不能表示某一具体数量．

      解：因为百分数表示一个数是另一个数的百分之几，是不能带单位的；所以一根绳子长#latex(`80%`)米的说法是错误的；

      故答案为：×

      本题主要考查了百分数的意义及表示方法，理解百分数不能带单位是解题的关键．],
  ),
  sc(
    [一个圆锥的底面半径扩大到原来的#latex(`2`)倍，高也扩大到原来的#latex(`2`)倍，它的体积扩大到原来的#klammern()倍．],
    (A: [#latex(`4`)], B: [#latex(`6`)], C: [#latex(`8`)], D: [不变]),
    difficulty: 3,
    business-type: "CSX-DANXUAN",
    answer: ([C],),
    analysis: [熟练掌握圆锥的体积公式，是解答此题的关键．

      根据圆锥体积公式，计算变化后的体积与原体积的比值即可求解．

      ∵ 圆锥体积公式为#latex(`V = \dfrac{1}{3} \pi r^2 h`)，

      设原圆锥底面半径为#latex(`r`)，高为#latex(`h`)，则原体积#latex(`V = \dfrac{1}{3} \pi r^2 h`)．

      变化后，底面半径变为 #latex(`2r`)，高变为 #latex(`2h`)，

      ∴ 新体积 #latex(`V' = \dfrac{1}{3} \pi (2r)^2 (2h) = \dfrac{1}{3} \pi \cdot 4r^2 \cdot 2h = \dfrac{1}{3} \pi \cdot 8r^2 h`)。

      ∴ #latex(`\dfrac{V'}{V} = \dfrac{\dfrac{1}{3} \pi \cdot 8r^2 h}{\dfrac{1}{3} \pi r^2 h} = 8`)．

      即体积扩大到原来的 #latex(`8`) 倍．

      故选：#latex(`\rm{C}`)．],
  ),
  judge(
    [一件衣服的价格先提价 #latex(`5\%`)，再降价 #latex(`5\%`)，价格仍是原价 （　　）],
    difficulty: 2,
    business-type: "CSX-PDT",
  ),
  judge([两种相关联的量不成正比例，就成反比例．（　　）], difficulty: 2, business-type: "CSX-PDT"),
  fb(
    [统计时使用折线统计图更能清楚的看出数量的变化情况． #blank_line() （判断对错）],
    difficulty: 2,
    business-type: "CSX-QITA",
    answer: ([对],),
    analysis: [本题考查了折线统计图的特点，熟练掌握各种统计图的特点是解题的关键；

      条形统计图能很容易看出数量的多少；折线统计图不仅容易看出数量的多少，而且能反映数量的增减变化情况；扇形统计图能反映部分与整体的关系；由此根据情况选择即可．

      解：折线统计图不仅可以看出数量多少，而且能清楚地看出数量的变化情况，

      故答案为：对．],
  ),
  title([三、选择题]),
  sc(
    [安妮花图书馆一本《安徒生童话》原价 #latex(`30`) 元，六一儿童节，八折出售，六一儿童节过后恢复原价，需要提价 #klammern()],
    (A: [#latex(`20\%`)], B: [#latex(`25\%`)], C: [#latex(`120\%`)], D: [#latex(`125\%`)]),
    difficulty: 2,
    business-type: "CSX-DANXUAN",
    answer: ([B],),
    analysis: [解：一本《安徒生童话》原价为 #latex(`30`) 元，八折出售的价格为：

      #latex(`30 \times 80\% = 24`)（元），

      六一儿童节过后恢复原价（即恢复到 #latex(`30`) 元），需要在现价 #latex(`24`) 元的基础上提价：

      #latex(`30 - 24 = 6`)（元），

      需要提价的百分比为：

      #latex(`\dfrac{6}{24} \times 100\% = 25\%`)．

      #latex(`\rm{A}`)．需要提价 #latex(`25\%`)，故本选项计算错误；

      #latex(`\rm{B}`)．需要提价 #latex(`25\%`)，故本选项计算正确；

      #latex(`\rm{C}`)．需要提价 #latex(`25\%`)，故本选项计算错误；

      #latex(`\rm{D}`)．需要提价 #latex(`25\%`)，故本选项计算错误．

      故选：#latex(`\rm{B}`)．],
  ),
  sc(
    [有一个小数 #latex(`7.12365365365365...`)，从小数点开始向右数 #latex(`64`) 个数字，#latex(`3`) 有 #klammern() 个．],
    (A: [#latex(`19`) 个], B: [#latex(`20`) 个], C: [#latex(`21`) 个], D: [#latex(`22`) 个]),
    difficulty: 3,
    business-type: "CSX-DANXUAN",
    answer: ([C],),
    analysis: [解：由题意可知，小数 #latex(`7.12365365365365...`) 是混循环小数，循环节是 #latex(`365`)．

      从小数点后第一位起，前两位数字是 #latex(`1`) 和 #latex(`2`)，不属于循环部分；

      从第三位起是循环节 #latex(`3, 6, 5`)，其长度为 #latex(`3`)．

      现在要统计前 #latex(`64`) 个数字中数字 #latex(`3`) 出现的次数，

      除去前两位，剩下数字的个数为 #latex(`64 - 2 = 62`) 个．

      因为 #latex(`62 \div 3 = 20 \cdots 2`)，

      说明这 #latex(`62`) 个数字中包含 #latex(`20`) 个完整的循环节，并且还剩下 #latex(`2`) 个数字．

      在每个完整的循环节 #latex(`3, 6, 5`) 中，数字 #latex(`3`) 出现 #latex(`1`) 次，因此在 #latex(`20`) 个循环节中数字 #latex(`3`) 出现 #latex(`20`) 次；

      余下的 #latex(`2`) 个数字依次是 #latex(`3, 6`)，其中数字 #latex(`3`) 出现 #latex(`1`) 次．

      所以，数字 #latex(`3`) 出现的总次数为 #latex(`20 + 1 = 21`) 次．

      #latex(`\rm{A}`)．#latex(`19`) 个，根据计算可知数字 #latex(`3`) 出现了 #latex(`21`) 次，故本选项计算错误；

      #latex(`\rm{B}`)．#latex(`20`) 个，根据计算可知数字 #latex(`3`) 出现了 #latex(`21`) 次，故本选项计算错误；

      #latex(`\rm{C}`)．#latex(`21`) 个，根据计算可知数字 #latex(`3`) 出现了 #latex(`21`) 次，故本选项计算正确；

      #latex(`\rm{D}`)．#latex(`22`) 个，根据计算可知数字 #latex(`3`) 出现了 #latex(`21`) 次，故本选项计算错误．

      故选：#latex(`\rm{C}`)．],
  ),
  sc(
    [#latex(`4a+8`)错写成#latex(`4\times (a+8)`)，结果比原来#klammern()．],
    (A: [多#latex(`4`)], B: [少#latex(`4`)], C: [多#latex(`24`)]),
    difficulty: 3,
    business-type: "CSX-DANXUAN",
    answer: ([C],),
    analysis: [本题考查了用乘法分配律计算含字母的算式，根据乘法分配律，将算式#latex(`4\times (a+8)`)去括号得出#latex(`4a+4\times 8`)，计算出结果，再求与#latex(`4a+8`)的差即可，要熟记运算律并能灵活使用．

      解：#latex(`4\times (a+8)-(4a+8)`)

      #latex(`=4a+32-4a-8`)

      #latex(`=32-8`)

      #latex(`=24`)

      答：结果比原来多#latex(`24`)．

      故选：#latex(`\rm{C}`)．],
  ),
  sc(
    [轮船向东偏北#latex(`30`)°航行，因有紧急任务，按顺时针方向调头 #latex(`90`)°去执行任务，那么这时轮船的航行方向是#klammern()],
    (
      A: [东偏南 #latex(`60`)°],
      B: [东偏南 #latex(`30`)°],
      C: [北偏西 #latex(`30`)°],
      D: [北偏西 #latex(`60`)°#linebreak()],
    ),
    difficulty: 2,
    business-type: "CSX-DANXUAN",
    answer: ([A],),
    analysis: [],
  ),
  sc(
    [某班的女生人数比全班人数的#latex(`\frac59`)少 #latex(`4`) 人，男生人数比全班人数的 #latex(`40%`)多 #latex(`6`) 人，那么这个班的男生人数比女生人数多#klammern()人.#linebreak()],
    (A: [#latex(`3`)], B: [#latex(`5`)], C: [#latex(`9`)], D: [#latex(`10`)]),
    difficulty: 3,
    business-type: "CSX-DANXUAN",
    answer: ([A],),
    analysis: [],
  ),
  title([四、计算题]),
  composite(
    [直接写出得数．],
    (
      fb(
        [#latex(`3 \div 10\% =`) #blank_line()],
        difficulty: 1,
        business-type: "CSX-JST",
        answer: ([#latex(`30`)],),
        analysis: [解：根据除以一个数等于乘上这个数的倒数（或者化为小数计算），可得：

          #latex(`3 \div 10\% = 3 \div 0.1 = 30`)．

          故答案为：#latex(`30`)．],
      ),
      fb(
        [#latex(`0.4^3 =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`0.064`)],),
        analysis: [解：根据小数乘法的计算方法，可得：

          #latex(`0.4^3 = 0.4 \times 0.4 \times 0.4 = 0.064`)．

          故答案为：#latex(`0.064`)．],
      ),
      fb(
        [#latex(`(\dfrac{5}{6} - \dfrac{3}{4}) \times 24 =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`2`)],),
        analysis: [解：根据乘法分配律计算如下：

          #latex(`(\dfrac{5}{6} - \dfrac{3}{4}) \times 24`)

          #latex(`= \dfrac{5}{6} \times 24 - \dfrac{3}{4} \times 24`)

          #latex(`= 20 - 18`)

          #latex(`= 2`)

          故答案为：#latex(`2`)．],
      ),
      fb(
        [#latex(`\dfrac{4}{7} + \dfrac{5}{11} + \dfrac{3}{7} - \dfrac{6}{11} =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`\dfrac{10}{11}`)],),
        analysis: [解：利用加法的交换律和结合律进行简便计算：

          #latex(`\dfrac{4}{7} + \dfrac{5}{11} + \dfrac{3}{7} - \dfrac{6}{11} = \left(\dfrac{4}{7} + \dfrac{3}{7}\right) + \left(\dfrac{5}{11} - \dfrac{6}{11}\right) = 1 - \dfrac{1}{11} = \dfrac{10}{11}`)．

          故答案为：#latex(`\dfrac{10}{11}`)．],
      ),
      fb(
        [#latex(`0.32 \times 25\% =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`0.08`)],),
        analysis: [解：根据小数乘百分数的计算方法，把百分数化成分数进行计算：

          #latex(`0.32 \times 25\% = 0.32 \times \dfrac{1}{4} = 0.08`)．

          故答案为：#latex(`0.08`)．],
      ),
      fb(
        [#latex(`1.2 : 96 =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`0.0125`)],),
        analysis: [解：根据小数除法的计算法则计算即可：

          #latex(`1.2 \div 96 = 0.0125`)（或 #latex(`1/80`)）．

          故答案为：#latex(`0.0125`)．],
      ),
      fb(
        [#latex(`4 \times \dfrac{3}{7} \div \dfrac{4}{9} =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`\dfrac{27}{7}`)],),
        analysis: [解：根据分数乘除混合运算的计算法则进行计算：

          #latex(`4 \times \dfrac{3}{7} \div \dfrac{4}{9} = 4 \times \dfrac{3}{7} \times \dfrac{9}{4} = \dfrac{27}{7}`)．

          故答案为：#latex(`\dfrac{27}{7}`)．],
      ),
      fb(
        [#latex(`\dfrac{3}{5} \times \dfrac{5}{3} \div \dfrac{7}{8} \times \dfrac{8}{7} =`) #blank_line()],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`\dfrac{64}{49}`)],),
        analysis: [解：根据分数乘除混合运算的顺序，从左到右依次计算：

          #latex(`\dfrac{3}{5} \times \dfrac{5}{3} \div \dfrac{7}{8} \times \dfrac{8}{7}`)

          #latex(`= 1 \div \dfrac{7}{8} \times \dfrac{8}{7}`)

          #latex(`= \dfrac{8}{7} \times \dfrac{8}{7}`)

          #latex(`= \dfrac{64}{49}`)

          故答案为：#latex(`\dfrac{64}{49}`)．],
      ),
    ),
    difficulty: 2,
    business-type: "CSX-JST",
  ),
  composite(
    [脱式计算，能简算的要简算．],
    (
      subj(
        [#latex(`8 - \dfrac{3}{2} \times \dfrac{10}{21} - \dfrac{2}{7}`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`7`)],),
        analysis: [解：原式#latex(`= 8 - \dfrac{3}{2} \times \dfrac{10}{21} - \dfrac{2}{7}`)

          #latex(`= 8 - \dfrac{5}{7} - \dfrac{2}{7}`)

          #latex(`= 8 - ( \dfrac{5}{7} + \dfrac{2}{7} )`)

          #latex(`= 8 - 1`)

          #latex(`= 7`)．],
      ),
      subj(
        [#latex(`\dfrac{8}{9} \times \left[ \dfrac{3}{4} - \left( \dfrac{7}{10} - \dfrac{1}{4} \right) \right]`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`\dfrac{4}{15}`)],),
        analysis: [解：#latex(`\dfrac{8}{9} \times \left[ \dfrac{3}{4} - \left( \dfrac{7}{10} - \dfrac{1}{4} \right) \right]`)

          #latex(`= \dfrac{8}{9} \times \left[ \dfrac{3}{4} - \left( \dfrac{14}{20} - \dfrac{5}{20} \right) \right]`)

          #latex(`= \dfrac{8}{9} \times \left[ \dfrac{3}{4} - \dfrac{9}{20} \right]`)

          #latex(`= \dfrac{8}{9} \times \left[ \dfrac{15}{20} - \dfrac{9}{20} \right]`)

          #latex(`= \dfrac{8}{9} \times \dfrac{6}{20}`)

          #latex(`= \dfrac{8}{9} \times \dfrac{3}{10}`)

          #latex(`= \dfrac{4}{15}`)．],
      ),
      subj(
        [#latex(`9.5 \times \dfrac{4}{5} + 0.2 \times 9.5`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`9.5`)],),
        analysis: [解：#latex(`9.5 \times \dfrac{4}{5} + 0.2 \times 9.5`)

          #latex(`= 9.5 \times 0.8 + 0.2 \times 9.5`)

          #latex(`= 9.5 \times (0.8 + 0.2)`)

          #latex(`= 9.5 \times 1`)

          #latex(`= 9.5`)．],
      ),
      subj(
        [#latex(`(1.25 \times 8 - 1.5) \times 2`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`17`)],),
        analysis: [解：#latex(`(1.25 \times 8 - 1.5) \times 2`)

          #latex(`= (10 - 1.5) \times 2`)

          #latex(`= 8.5 \times 2`)

          #latex(`= 17`)．],
      ),
      subj(
        [#latex(`1010 - 3072 \div 12`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`754`)],),
        analysis: [解：#latex(`1010 - 3072 \div 12`)

          #latex(`= 1010 - 256`)

          #latex(`= 754`)．],
      ),
      subj(
        [#latex(`1.25 \times 4 \times 0.8 \times 0.25`)],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`1`)],),
        analysis: [解：#latex(`\because`) 原式为 #latex(`1.25 \times 4 \times 0.8 \times 0.25`)，

          #latex(`\therefore`) 根据乘法交换律与结合律，原式可变形为：

          原式 #latex(`= (1.25 \times 0.8) \times (4 \times 0.25)`)，

          #latex(`\because 1.25 \times 0.8 = 1`)，

          #latex(`\because 4 \times 0.25 = 1`)，

          #latex(`\therefore`) 原式 #latex(`= 1 \times 1`)，

          #latex(`\therefore`) 原式 #latex(`= 1`)．

          故答案为：#latex(`1`)．],
      ),
    ),
    difficulty: 3,
    business-type: "CSX-ZHT",
  ),
  composite(
    [解方程或比例．],
    (
      subj(
        [#latex(`\dfrac{3}{5} = \dfrac{4.2}{x}`)],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`x = 7`)],),
        analysis: [解：#latex(`\because \dfrac{3}{5} = \dfrac{4.2}{x}`)，

          #latex(`\therefore 3x = 4.2 \times 5`)，

          (不对)

          #latex(`\therefore 3x = 21`)，

          #latex(`\therefore x = 7`)．],
      ),
      subj(
        [#latex(`1 - \dfrac{3}{5}x = \dfrac{3}{10}`)],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`x = \dfrac{7}{6}`)],),
        analysis: [解：#latex(`1 - \dfrac{3}{5}x = \dfrac{3}{10}`)，

          两边同时乘以 #latex(`10`)得：

          #latex(`10 - 6x = 3`)，

          移项得：

          #latex(`-6x = 3 - 10`)，

          合并同类项得：

          #latex(`-6x = -7`)，

          系数化为 #latex(`1`)得：

          #latex(`x = \dfrac{7}{6}`)．],
      ),
      subj(
        [#latex(`\dfrac{1}{6} : x = \dfrac{1}{10} : \dfrac{1}{4}`)],
        difficulty: 2,
        business-type: "CSX-JST",
        answer: ([#latex(`x = \dfrac{5}{12}`)],),
        analysis: [解：根据比例的基本性质，两外项之积等于两内项之积，

          #latex(`\because \dfrac{1}{6} : x = \dfrac{1}{10} : \dfrac{1}{4}`)，

          #latex(`\therefore \dfrac{1}{10}x = \dfrac{1}{6} \times \dfrac{1}{4}`)，

          #latex(`\dots \dfrac{1}{10}x = \dfrac{1}{24}`)，

          #latex(`\therefore x = \dfrac{1}{24} \times 10`)，

          #latex(`\dots x = \dfrac{5}{12}`)．],
      ),
    ),
    difficulty: 2,
    business-type: "CSX-JST",
  ),
  title([五、作图题]),
  subj(
    [在方格纸上按要求画图．

      (#latex(`1`)) 画出图形 #latex(`A`) 绕点 #latex(`O`) 逆时针旋转 #latex(`90^{\circ}`) 得到的图形 #latex(`B`)．

      (#latex(`2`)) 画出图形 #latex(`A`) 关于直线 #latex(`l`) 的轴对称图形 #latex(`C`)．

      (#latex(`3`)) 画出图形 #latex(`C`) 向右平移 #latex(`4`) 格后的图形 #latex(`D`)．

      (#latex(`4`)) 画出图形 #latex(`A`) 按 #latex(`2 : 1`) 放大后的图形 #latex(`E`)．

      #box(baseline: 25%, image(
        "https://img.xkw.com/dksih/QBM/2019/6/1/2216198352404480/2217167164645376/STEM/3eea9d4ea59f4fc6814c6f31af81e94e.png",
        width: 100%,
      ))],
    difficulty: 3,
    business-type: "CSX-JIEDA",
    answer: (
      [绕点 #latex(`O`) 逆时针旋转 #latex(`90^{\circ}`) 得到的图形 #latex(`\rm{B}`) 如图所示；

        (#latex(`2`)) 关于直线 #latex(`l`) 的轴对称图形 #latex(`\rm{C}`) 如图所示；

        (#latex(`3`)) 向右平移 #latex(`4`) 格后的图形 #latex(`\rm{D}`) 如图所示；

        (#latex(`4`)) 按 #latex(`2 : 1`) 放大后的图形 #latex(`\rm{E}`) 如图所示．

        #box(baseline: 25%, image(
          "https://tiku-1252350207.cos.ap-beijing.myqcloud.com/k12-paper/2-1780751126533.png",
          width: 100%,
        ))],
    ),
    analysis: [解：(#latex(`1`)) #latex(`\because`) 图形 #latex(`\rm{A}`) 的直角顶点为 #latex(`O(4,4)`)，另外两个顶点分别为 #latex(`(4,6)`)，#latex(`(5,4)`)，

      #latex(`\because`) 图形 #latex(`\rm{B}`) 是图形 #latex(`\rm{A}`) 绕点 #latex(`O`) 逆时针旋转 #latex(`90^{\circ}`) 得到的图形，

      #latex(`\therefore`) 旋转中心 #latex(`O(4,4)`) 的位置保持不变，

      #latex(`\because`) 顶点 #latex(`(4,6)`) 绕点 #latex(`O`) 逆时针旋转 #latex(`90^{\circ}`) 后的对应点为 #latex(`(2,4)`)，

      #latex(`\because`) 顶点 #latex(`(5,4)`) 绕点 #latex(`O`) 逆时针旋转 #latex(`90^{\circ}`) 后的对应点为 #latex(`(4,5)`)，

      #latex(`\therefore`) 顺次连接点 #latex(`(4,4)`)，#latex(`(2,4)`)，#latex(`(4,5)`) 即可得到图形 #latex(`\rm{B}`)．

      (#latex(`2`))

      #latex(`\because`) 对称轴直线 #latex(`l`) 所在的直线为 #latex(`x=6`)，

      #latex(`\because`) 图形 #latex(`\rm{A}`) 的三个顶点坐标分别为 #latex(`(4,4)`)，#latex(`(4,6)`)，#latex(`(5,4)`)，

      #latex(`\therefore`) 各顶点关于直线 #latex(`l`) 对称的对应点坐标分别为 #latex(`(8,4)`)，#latex(`(8,6)`)，#latex(`(7,4)`)，

      #latex(`\therefore`) 顺次连接点 #latex(`(8,4)`)，#latex(`(8,6)`)，#latex(`(7,4)`) 即可得到图形 #latex(`\rm{C}`)．

      (#latex(`3`))

      #latex(`\because`) 图形 #latex(`\rm{D}`) 是图形 #latex(`\rm{C}`) 向右平移 #latex(`4`) 格后得到的图形，

      #latex(`\because`) 图形 #latex(`\rm{C}`) 的三个顶点坐标分别为 #latex(`(8,4)`)，#latex(`(8,6)`)，#latex(`(7,4)`)，

      #latex(`\therefore`) 向右平移 #latex(`4`) 格后，对应顶点的横坐标均增加 #latex(`4`)，纵坐标保持不变，

      #latex(`\therefore`) 各对应顶点的坐标分别为 #latex(`(12,4)`)，#latex(`(12,6)`)，#latex(`(11,4)`)，

      #latex(`\therefore`) 顺次连接点 #latex(`(12,4)`)，#latex(`(12,6)`)，#latex(`(11,4)`) 即可得到图形 #latex(`\rm{D}`)．

      (#latex(`4`))

      #latex(`\because`) 图形 #latex(`\rm{E}`) 是图形 #latex(`\rm{A}`) 按 #latex(`2 : 1`) 放大后的图形，

      #latex(`\because`) 图形 #latex(`\rm{A}`) 是直角边分别为 #latex(`2`) 和 #latex(`1`) 的直角三角形，

      #latex(`\therefore`) 图形 #latex(`\rm{E}`) 是直角边分别为 #latex(`4`) 和 #latex(`2`) 的直角三角形，

      #latex(`\therefore`) 在方格纸的合适位置（如以 #latex(`(1,1)`) 为直角顶点）画出直角边长分别为 #latex(`4`) 和 #latex(`2`) 的直角三角形，即可得到图形 #latex(`\rm{E}`)．],
  ),
  title([六、填空题]),
  fb(
    [小明按照如图的方法用灰色和白色的小正方形摆图形．当中间摆#latex(`2022`)个灰色的小正方形时，四周共需要摆白色小正方形的个数为#blank_line() ．

      #box(baseline: 25%, image(
        "https://k12static.xdf.cn/k12/xkw/1748877285661/c7d55756-6139-4e68-b30e-ba314c40a525.png",
        width: 60.98%,
      ))],
    difficulty: 3,
    business-type: "CSX-TIANKONG",
    answer: ([#latex(`4050`)],),
    analysis: [给图形依次标上序号，观察图形规律可得：灰色正方形的个数与图形的序号相同，白色正方形的个数比灰色正方形个数的#latex(`2`)倍多#latex(`6`)，据此规律即可得出答案．

      解：根据题意分析可得：

      灰色正方形的个数与图形的序号数相同，

      第#latex(`1`)幅图中白色正方形有#latex(`2\times 1+6=8`)​(个)；

      第#latex(`2`)幅图中白色正方形有#latex(`2\times 2+6=10`)​(个)；

      第#latex(`3`)幅图中白色正方形有#latex(`2\times 3+6=12`)​(个)；

      …

      第#latex(`n`)幅图中白色正方形有#latex(`\left(2n+6\right)`)个；

      #latex(`\therefore`)当中间摆#latex(`2022`)个灰色的小正方形时，也就是第#latex(`2022`)幅图，

      其中白色正方形的个数为：#latex(`2\times 2022+6=4050`) (个)，

      故答案为：#latex(`4050`)．

      本题考查图形的规律问题，找出每个图形中灰色正方形和白色正方形的个数与序号间的关系是解题的关键．],
  ),
  title([七、解答题]),
  subj(
    [求下图阴影部分的面积．

      #box(baseline: 25%, image(
        "https://img.xkw.com/dksih/QBM/2019/6/1/2216198352404480/2217167164653569/STEM/def71d3f-c1b7-493e-8afe-8ebcb81cafd8.png",
        width: 100%,
      ))],
    difficulty: 3,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`24 - 4\pi`)],),
    analysis: [解：

      连接 #latex(`OE`) ．

      #latex(`\because`) 四边形 #latex(`ABCD`) 是正方形，边长为 #latex(`8`)，

      #latex(`\therefore \angle DAB = 90^{\circ}`)，#latex(`AB = BC = 8`)，

      #latex(`\because AC`) 是正方形 #latex(`ABCD`) 的对角线，

      #latex(`\therefore \angle BAC = 45^{\circ}`)，

      #latex(`\because O`) 是 #latex(`AB`) 的中点，以 #latex(`AB`) 为直径在正方形内作半圆，

      #latex(`\therefore OA = OB = OE = 4`)，

      #latex(`\therefore \angle OAE = \angle OEA = 45^{\circ}`)，

      #latex(`\therefore \angle AOE = 180^{\circ} - \angle OAE - \angle OEA = 90^{\circ}`)，

      #latex(`\therefore \angle BOE = 180^{\circ} - \angle AOE = 90^{\circ}`)，

      #latex(`\therefore S_{\triangle ABC} = \dfrac{1}{2} \times AB \times BC = \dfrac{1}{2} \times 8 \times 8 = 32`)，

      #latex(`\therefore S_{\triangle AOE} = \dfrac{1}{2} \times OA \times OE = \dfrac{1}{2} \times 4 \times 4 = 8`)，

      #latex(`\because \angle BOE = 90^{\circ}`)，

      #latex(`\therefore S_{\text{扇形 } BOE} = \dfrac{90}{360} \times \pi \times OE^2 = \dfrac{1}{4} \times \pi \times 4^2 = 4\pi`)，

      #latex(`\because S_{\triangle ABC} = S_{\triangle AOE} + S_{\text{扇形 } BOE} + S_{\text{阴影}}`)，

      #latex(`\therefore S_{\text{阴影}} = S_{\triangle ABC} - S_{\triangle AOE} - S_{\text{扇形 } BOE}`)

      #latex(`\therefore S_{\text{阴影}} = 32 - 8 - 4\pi = 24 - 4\pi`)．

      故答案为：#latex(`24 - 4\pi`)．],
  ),
  subj(
    [只列综合算式，不计算．

      南洋酒店第一季度的营业额中应纳税的部分按 #latex(`3\%`) 纳税，税后余额为 #latex(`190`) 万元，第一季度纳税多少万元？],
    difficulty: 3,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`190 \div (1 - 3\%) \times 3\%`)],),
    analysis: [解：#latex(`\because`) 南洋酒店第一季度的营业额中应纳税的部分按 #latex(`3\%`) 纳税，

      #latex(`\therefore`) 税后余额占应纳税部分的百分比为 #latex(`1 - 3\%`)，

      #latex(`\because`) 税后余额为 #latex(`190`) 万元，

      #latex(`\therefore`) 应纳税的部分为 #latex(`190 \div (1 - 3\%)`) 万元，

      #latex(`\therefore`) 第一季度纳税金额为 #latex(`190 \div (1 - 3\%) \times 3\%`) 万元．

      故答案为：#latex(`190 \div (1 - 3\%) \times 3\%`)．],
  ),
  subj(
    [一个圆锥形黄沙堆．底面周长为#latex(`25.12\mathrm{m}`)．高#latex(`3\mathrm{m}`)．每立方米黄沙重#latex(`1.49\mathrm{t}`)，这个黄沙堆共重多少吨？（#latex(`\pi =3.14`)）],
    difficulty: 2,
    business-type: "CSX-JIEDA",
    answer: ([这个黄沙堆共重 #latex(`74.8576`)吨],),
    analysis: [本题考查了圆锥的体积公式，根据圆锥的体积公式计算即可得解，理解题意，正确列式计算是解此题的关键．

      解：#latex(`\dfrac{1}{3}\times 3.14\times \left(25.12\div 3.14\div 2\right)^{2}\times 3\times 1.49=\dfrac{1}{3}\times 3.14\times 16\times 3\times 1.49=74.8576`)（吨），

      故这个黄沙堆共重#latex(`74.8576`)吨．],
  ),
  subj(
    [只列综合算式，不计算．

      春节过后，亮亮把自己的 #latex(`1500`) 元压岁钱存入银行，存期三年，年利率为 #latex(`3.25\%`)，到期后亮亮可得本金和利息共多少元？（免征利息税）],
    difficulty: 2,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`1500 + 1500 \times 3.25\% \times 3`)（或 #latex(`1500 \times (1 + 3.25\% \times 3)`)）],),
    analysis: [解：#latex(`\because`) 本金为 #latex(`1500`) 元，年利率为 #latex(`3.25\%`)，存期为 #latex(`3`) 年，

      #latex(`\therefore`) 到期时亮亮可以获得的利息为 #latex(`1500 \times 3.25\% \times 3`) 元，

      #latex(`\therefore`) 到期后亮亮可得本金和利息共为 #latex(`1500 + 1500 \times 3.25\% \times 3`) 元（或 #latex(`1500 \times (1 + 3.25\% \times 3)`) 元）．

      故列综合算式为：#latex(`1500 + 1500 \times 3.25\% \times 3`)（或 #latex(`1500 \times (1 + 3.25\% \times 3)`)）．],
  ),
  subj(
    [北京和南京大约相距 #latex(`1050`) 千米，一辆快车和一辆动车同时从两地出发，相向而行，#latex(`3`) 小时相遇，动车与快车的速度比是 #latex(`5 : 2`)，求动车每小时行多少千米．],
    difficulty: 3,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`250`)],),
    analysis: [解：#latex(`\because`) 北京和南京大约相距 #latex(`1050`) 千米，两车相向而行，#latex(`3`) 小时相遇，

      #latex(`\therefore`) 两车的速度之和为 #latex(`1050 \div 3 = 350`)（千米\/小时），

      #latex(`\because`) 动车与快车的速度比是 #latex(`5 : 2`)，

      #latex(`\therefore`) 动车的速度占两车速度之和的 #latex(`\dfrac{5}{5 + 2}`)，

      #latex(`\therefore`) 动车每小时行：#latex(`350 \times \dfrac{5}{5 + 2} = 250`)（千米）．

      答：动车每小时行 #latex(`250`) 千米．],
  ),
  subj(
    [某工程队铺一段路，原计划每天铺#latex(`9.6`)千米，#latex(`15`)天铺完，实际每天比原计划多铺#latex(`2.4`)千米，实际要用多少天铺完？（列方程解答）],
    difficulty: 2,
    business-type: "CSX-JIEDA",
    answer: ([见解析],),
    analysis: [解：设实际要用x天铺完，#linebreak()（#latex(`2.4`)+#latex(`9.6`)）#latex(`\rm{x}`)=#latex(`9.6`)×#latex(`15`)，#linebreak()解得：#latex(`\rm{x}`)=#latex(`12`)，#linebreak()答：实际要用#latex(`12`)天铺完．],
  ),
  subj(
    [图书室有科技书 #latex(`1200`) 本，科技书比文艺书的 #latex(`2`) 倍少 #latex(`150`) 本，文艺书有多少本？（用方程解答）],
    difficulty: 2,
    business-type: "CSX-JIEDA",
    answer: ([文艺书有 #latex(`675`) 本．],),
    analysis: [解：设文艺书有 #latex(`x`) 本，

      #latex(`\because`) 科技书比文艺书的 #latex(`2`) 倍少 #latex(`150`) 本，且科技书有 #latex(`1200`) 本，

      #latex(`\therefore 2x - 150 = 1200`)，

      #latex(`2x = 1200 + 150`)，

      #latex(`2x = 1350`)，

      #latex(`x = 675`)．

      答：文艺书有 #latex(`675`) 本．],
  ),
  subj(
    [为迎接省文明城市创建，新区拓宽一条公路，第一天修了#latex(`15\mathrm{％}`)，第二天比第一天少修了#latex(`300`)米，还剩#latex(`75\mathrm{％}`)，这条公路全长多少米？],
    difficulty: 1,
    business-type: "CSX-JIEDA",
    answer: ([6000米],),
    analysis: [本题主要考查了百分数的实际应用，先求出第二天比第一天少修的百分比，再根据第二天比第一天少修#latex(`300`)米列式求解即可．

      解：#latex(`300\div \left[15\mathrm{\% }-\left(1-15\mathrm{\% }-75\mathrm{\% }\right)\right]=6000`)米，

      答：这条公路全长#latex(`6000`)米．],
  ),
  subj(
    [中国参加第 #latex(`31`) 届里约奥运会的运动员中，男运动员占运动员总数的 #latex(`\dfrac{5}{13}`)，女运动员比运动员总数的 #latex(`\dfrac{5}{8}`) 少 #latex(`4`) 人．中国参加第 #latex(`31`) 届奥运会的运动员共有多少人？],
    difficulty: 3,
    business-type: "CSX-JIEDA",
    answer: ([#latex(`416`)],),
    analysis: [解：设中国参加第 #latex(`31`) 届奥运会的运动员共有 #latex(`x`) 人．

      #latex(`\because`) 男运动员占运动员总数的 #latex(`\dfrac{5}{13}`)，

      #latex(`\therefore`) 男运动员人数为 #latex(`\dfrac{5}{13}x`) 人．

      #latex(`\because`) 女运动员比运动员总数的 #latex(`\dfrac{5}{8}`) 少 #latex(`4`) 人，

      #latex(`\therefore`) 女运动员人数为 #latex(`\left(\dfrac{5}{8}x - 4\right)`) 人．

      #latex(`\because`) 男运动员人数与女运动员人数之和等于总人数，

      #latex(`\therefore`) #latex(`\dfrac{5}{13}x + \dfrac{5}{8}x - 4 = x`)，

      #latex(`\therefore`) #latex(`\left(\dfrac{5}{13} + \dfrac{5}{8} - 1\right)x = 4`)，

      #latex(`\therefore`) #latex(`\left(\dfrac{40}{104} + \dfrac{65}{104} - \dfrac{104}{104}\right)x = 4`)，

      #latex(`\therefore`) #latex(`\dfrac{1}{104}x = 4`)，

      #latex(`\therefore`) #latex(`x = 416`)．

      答：中国参加第 #latex(`31`) 届奥运会的运动员共有 #latex(`416`) 人．],
  ),
  composite(
    [笑笑过生日，有 #latex(`6`) 位小伙伴来做客．她用一大盒果汁招待同学，给每位同学倒上一杯后，剩下的倒给自己．（果汁包装盒和杯子如下图，厚度忽略不计）

      #box(baseline: 25%, image(
        "https://img.xkw.com/dksih/QBM/2020/6/24/2491735018135552/2498837784051712/STEM/7d960ac61b6a4dbb9b9a79d7eb27c743.png",
        width: 100%,
      ))],
    (
      subj(
        [这盒果汁有多少毫升？],
        difficulty: 2,
        business-type: "CSX-ZHT",
        answer: ([#latex(`1320`) 毫升],),
        analysis: [解：#latex(`\because`) 由图可知，果汁包装盒是一个长方体，其长为 #latex(`11\text{ cm}`)，宽为 #latex(`6\text{ cm}`)，高为 #latex(`20\text{ cm}`)，

          #latex(`\therefore`) 这盒果汁的体积为：

          #latex(`11 \times 6 \times 20 = 1320\text{ (cm}^3\text{)}`)，

          #latex(`\because 1\text{ cm}^3 = 1\text{ mL}`)，

          #latex(`\therefore 1320\text{ cm}^3 = 1320\text{ mL}`)．

          故答案为：#latex(`1320`) 毫升．],
      ),
      subj(
        [每位小客人喝了多少毫升？],
        difficulty: 2,
        business-type: "CSX-ZHT",
        answer: ([#latex(`282.6`)],),
        analysis: [解：根据题意和图示可知，圆柱形杯子的底面直径为 #latex(`6\text{ cm}`)，高为 #latex(`10\text{ cm}`)，

          #latex(`\because`) 圆柱形杯子的底面半径为：

          #latex(`r = 6 \div 2 = 3\text{ cm}`)，

          #latex(`\therefore`) 圆柱形杯子的容积为：

          #latex(`V = 3.14 \times 3^2 \times 10 = 282.6\text{ cm}^3`)，

          #latex(`\because 1\text{ cm}^3 = 1\text{ mL}`)，

          #latex(`\therefore 282.6\text{ cm}^3 = 282.6\text{ mL}`)，

          #latex(`\therefore`) 每位小客人喝了 #latex(`282.6`) 毫升．],
      ),
      subj(
        [笑笑喝了这盒果汁的几分之几？],
        difficulty: 3,
        business-type: "CSX-ZHT",
        answer: ([#latex(`\dfrac{587}{2000}`)（或 #latex(`29.35\%`)）],),
        analysis: [解：#latex(`\because`) 果汁包装盒是长方体，长为 #latex(`15\text{ cm}`)，宽为 #latex(`8\text{ cm}`)，高为 #latex(`20\text{ cm}`)，

          #latex(`\therefore`) 这盒果汁的总容积为：#latex(`V_{\text{总}} = 15 \times 8 \times 20 = 2400\text{ cm}^3 = 2400\text{ mL}`)，

          #latex(`\because`) 杯子是圆柱形，底面直径为 #latex(`6\text{ cm}`)，高为 #latex(`10\text{ cm}`)，

          #latex(`\therefore`) 杯子的底面半径为：#latex(`r = 6 \div 2 = 3\text{ cm}`)，

          #latex(`\therefore`) 一个杯子的容积为：#latex(`V_{\text{杯}} = 3.14 \times 3^2 \times 10 = 282.6\text{ cm}^3 = 282.6\text{ mL}`)，

          #latex(`\because`) 有 #latex(`6`) 位小伙伴，给每位同学倒满一杯，

          #latex(`\therefore`) 一共倒出的果汁体积为：#latex(`282.6 \times 6 = 1695.6\text{ mL}`)，

          #latex(`\therefore`) 剩下的倒给自己，笑笑喝了的果汁体积为：#latex(`2400 - 1695.6 = 704.4\text{ mL}`)，

          #latex(`\therefore`) 笑笑喝了这盒果汁的比例为：#latex(`704.4 \div 2400 = 0.2935 = 29.35\%`)，

          #latex(`\therefore`) 化为最简分数为：#latex(`\dfrac{704.4}{2400} = \dfrac{7044}{24000} = \dfrac{587}{2000}`)．

          故答案为：#latex(`\dfrac{587}{2000}`)（或 #latex(`29.35\%`)）．],
      ),
    ),
    difficulty: 3,
    business-type: "CSX-ZHT",
  ),
)

#let metadata = (
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913149181935616",
    questionId: "3625913149181935616",
    paperId: "3625849114759385088",
    questionIndex: 1,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913153896173568",
    questionId: "3625913153896173568",
    paperId: "3625849114759385088",
    questionIndex: 1,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913159328735232",
    questionId: "3625913159328735232",
    paperId: "3625849114759385088",
    questionIndex: 2,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3184611963893026818",
    questionId: "3184611963893026818",
    paperId: "3625849114759385088",
    questionIndex: 3,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("分数的认识", "分数的运算"),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913169314594816",
    questionId: "3625913169314594816",
    paperId: "3625849114759385088",
    questionIndex: 4,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913174632972288",
    questionId: "3625913174632972288",
    paperId: "3625849114759385088",
    questionIndex: 5,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913179699691520",
    questionId: "3625913179699691520",
    paperId: "3625849114759385088",
    questionIndex: 6,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
    smallQuestions: (
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913179699691520",
        questionId: "3625913179901018112",
        paperId: "3625849114759385088",
        questionIndex: 1,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913179699691520",
        questionId: "3625913179901018113",
        paperId: "3625849114759385088",
        questionIndex: 2,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
    ),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913186544635904",
    questionId: "3625913186544635904",
    paperId: "3625849114759385088",
    questionIndex: 7,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3107712143599636480",
    questionId: "3107712143599636480",
    paperId: "3625849114759385088",
    questionIndex: 8,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("规律型：图形的变化类",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913196156329984",
    questionId: "3625913196156329984",
    paperId: "3625849114759385088",
    questionIndex: 9,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913201493295104",
    questionId: "3625913201493295104",
    paperId: "3625849114759385088",
    questionIndex: 10,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3063194952777080832",
    questionId: "3063194952777080832",
    paperId: "3625849114759385088",
    questionIndex: 11,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("圆柱和圆锥",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913211106639872",
    questionId: "3625913211106639872",
    paperId: "3625849114759385088",
    questionIndex: 12,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913216139644928",
    questionId: "3625913216139644928",
    paperId: "3625849114759385088",
    questionIndex: 14,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3439893741787168768",
    questionId: "3439893741787168768",
    paperId: "3625849114759385088",
    questionIndex: 13,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("其他",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3460424511759630336",
    questionId: "3460424511759630336",
    paperId: "3625849114759385088",
    questionIndex: 14,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("其他",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913230113415168",
    questionId: "3625913230113415168",
    paperId: "3625849114759385088",
    questionIndex: 15,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913235431792640",
    questionId: "3625913235431792640",
    paperId: "3625849114759385088",
    questionIndex: 16,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3160160095361052672",
    questionId: "3160160095361052672",
    paperId: "3625849114759385088",
    questionIndex: 17,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("统计图的选择", "折线统计图"),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913245112246272",
    questionId: "3625913245112246272",
    paperId: "3625849114759385088",
    questionIndex: 20,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913249509527552",
    questionId: "3625913249509527552",
    paperId: "3625849114759385088",
    questionIndex: 18,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913254541041664",
    questionId: "3625913254541041664",
    paperId: "3625849114759385088",
    questionIndex: 19,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "2756263854441779204",
    questionId: "2756263854441779204",
    paperId: "3625849114759385088",
    questionIndex: 20,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("整式的加减（计算）",),
  ),
  (
    questionSource: 1,
    questionType: "1",
    relationType: 1,
    parentId: "2141235345487638528",
    questionId: "2141235345487638528",
    paperId: "3625849114759385088",
    questionIndex: 21,
    sysCode: 9,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("方向角",),
  ),
  (
    questionSource: 1,
    questionType: "1",
    relationType: 1,
    parentId: "2139879511991357440",
    questionId: "2139879511991357440",
    paperId: "3625849114759385088",
    questionIndex: 22,
    sysCode: 9,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("其他问题（一元一次方程）",),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913273752764416",
    questionId: "3625913273752764416",
    paperId: "3625849114759385088",
    questionIndex: 26,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913278165172224",
    questionId: "3625913278165172224",
    paperId: "3625849114759385088",
    questionIndex: 23,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
    smallQuestions: (
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498816",
        paperId: "3625849114759385088",
        questionIndex: 1,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498817",
        paperId: "3625849114759385088",
        questionIndex: 2,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498818",
        paperId: "3625849114759385088",
        questionIndex: 3,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498819",
        paperId: "3625849114759385088",
        questionIndex: 4,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498820",
        paperId: "3625849114759385088",
        questionIndex: 5,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498821",
        paperId: "3625849114759385088",
        questionIndex: 6,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498822",
        paperId: "3625849114759385088",
        questionIndex: 7,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913278165172224",
        questionId: "3625913278366498823",
        paperId: "3625849114759385088",
        questionIndex: 8,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
    ),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913286587174912",
    questionId: "3625913286587174912",
    paperId: "3625849114759385088",
    questionIndex: 24,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
    smallQuestions: (
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501504",
        paperId: "3625849114759385088",
        questionIndex: 1,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501505",
        paperId: "3625849114759385088",
        questionIndex: 2,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501506",
        paperId: "3625849114759385088",
        questionIndex: 3,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501507",
        paperId: "3625849114759385088",
        questionIndex: 4,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501508",
        paperId: "3625849114759385088",
        questionIndex: 5,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913286587174912",
        questionId: "3625913286788501509",
        paperId: "3625849114759385088",
        questionIndex: 6,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
    ),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913293046403072",
    questionId: "3625913293046403072",
    paperId: "3625849114759385088",
    questionIndex: 25,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
    smallQuestions: (
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913293046403072",
        questionId: "3625913293247729664",
        paperId: "3625849114759385088",
        questionIndex: 1,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913293046403072",
        questionId: "3625913293247729665",
        paperId: "3625849114759385088",
        questionIndex: 2,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913293046403072",
        questionId: "3625913293247729666",
        paperId: "3625849114759385088",
        questionIndex: 3,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
    ),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913298245689344",
    questionId: "3625913298245689344",
    paperId: "3625849114759385088",
    questionIndex: 30,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913302609416192",
    questionId: "3625913302609416192",
    paperId: "3625849114759385088",
    questionIndex: 26,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913307773542400",
    questionId: "3625913307773542400",
    paperId: "3625849114759385088",
    questionIndex: 32,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3084857810862485504",
    questionId: "3084857810862485504",
    paperId: "3625849114759385088",
    questionIndex: 27,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("规律型：图形的变化类",),
  ),
  (
    questionSource: 3,
    questionType: "2",
    relationType: 0,
    parentId: "3625913320911708160",
    questionId: "3625913320911708160",
    paperId: "3625849114759385088",
    questionIndex: 34,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913326079090688",
    questionId: "3625913326079090688",
    paperId: "3625849114759385088",
    questionIndex: 28,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913331164237824",
    questionId: "3625913331164237824",
    paperId: "3625849114759385088",
    questionIndex: 29,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "2970416119154302976",
    questionId: "2970416119154302976",
    paperId: "3625849114759385088",
    questionIndex: 30,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("圆柱和圆锥",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913341480574976",
    questionId: "3625913341480574976",
    paperId: "3625849114759385088",
    questionIndex: 31,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913347236970496",
    questionId: "3625913347236970496",
    paperId: "3625849114759385088",
    questionIndex: 32,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "2365775845531144192",
    questionId: "2365775845531144192",
    paperId: "3625849114759385088",
    questionIndex: 33,
    sysCode: 12,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("行程问题（一元一次方程）", "行程问题（一元一次方程）"),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913357034864640",
    questionId: "3625913357034864640",
    paperId: "3625849114759385088",
    questionIndex: 34,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 1,
    parentId: "3379191167956639744",
    questionId: "3379191167956639744",
    paperId: "3625849114759385088",
    questionIndex: 35,
    sysCode: 13,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: ("其他",),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913366426689536",
    questionId: "3625913366426689536",
    paperId: "3625849114759385088",
    questionIndex: 36,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
  ),
  (
    questionSource: 2,
    questionType: "1",
    relationType: 0,
    parentId: "3625913371511791616",
    questionId: "3625913371511791616",
    paperId: "3625849114759385088",
    questionIndex: 37,
    sysCode: 1,
    standardFlag: "",
    inputType: none,
    addFlag: none,
    flagUse: "",
    knwNames: (),
    smallQuestions: (
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913371511791616",
        questionId: "3625913371746672640",
        paperId: "3625849114759385088",
        questionIndex: 1,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913371511791616",
        questionId: "3625913371746672641",
        paperId: "3625849114759385088",
        questionIndex: 2,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
      (
        questionSource: 2,
        questionType: "",
        relationType: "",
        parentId: "3625913371511791616",
        questionId: "3625913371746672642",
        paperId: "3625849114759385088",
        questionIndex: 3,
        sysCode: 1,
        standardFlag: "",
        inputType: none,
        addFlag: none,
        flagUse: "",
        knwNames: (),
      ),
    ),
  ),
)

#(forrender, metadata)
