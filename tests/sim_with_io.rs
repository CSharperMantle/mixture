#![cfg(feature = "io")]
#![allow(clippy::all)]
#![allow(clippy::unwrap_used)]

use std::sync::RwLock;

use mixture::*;

static PRIMES_OUTPUT: RwLock<String> = RwLock::new(String::new());
const PRIMES_OUTPUT_EXPECTED: &'static str = r#"FIRST|FIVE|HUNDRED|PRIMES|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0000|0233|0547|0877|1229|1597|1993|2371|2749|3187||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0003|0239|0557|0881|1231|1601|1997|2377|2753|3191||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0005|0241|0563|0883|1237|1607|1999|2381|2767|3203||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0007|0251|0569|0887|1249|1609|2003|2383|2777|3209||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0011|0257|0571|0907|1259|1613|2011|2389|2789|3217||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0013|0263|0577|0911|1277|1619|2017|2393|2791|3221||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0017|0269|0587|0919|1279|1621|2027|2399|2797|3229||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0019|0271|0593|0929|1283|1627|2029|2411|2801|3251||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0023|0277|0599|0937|1289|1637|2039|2417|2803|3253||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0029|0281|0601|0941|1291|1657|2053|2423|2819|3257||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0031|0283|0607|0947|1297|1663|2063|2437|2833|3259||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0037|0293|0613|0953|1301|1667|2069|2441|2837|3271||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0041|0307|0617|0967|1303|1669|2081|2447|2843|3299||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0043|0311|0619|0971|1307|1693|2083|2459|2851|3301||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0047|0313|0631|0977|1319|1697|2087|2467|2857|3307||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0053|0317|0641|0983|1321|1699|2089|2473|2861|3313||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0059|0331|0643|0991|1327|1709|2099|2477|2879|3319||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0061|0337|0647|0997|1361|1721|2111|2503|2887|3323||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0067|0347|0653|1009|1367|1723|2113|2521|2897|3329||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0071|0349|0659|1013|1373|1733|2129|2531|2903|3331||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0073|0353|0661|1019|1381|1741|2131|2539|2909|3343||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0079|0359|0673|1021|1399|1747|2137|2543|2917|3347||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0083|0367|0677|1031|1409|1753|2141|2549|2927|3359||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0089|0373|0683|1033|1423|1759|2143|2551|2939|3361||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0097|0379|0691|1039|1427|1777|2153|2557|2953|3371||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0101|0383|0701|1049|1429|1783|2161|2579|2957|3373||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0103|0389|0709|1051|1433|1787|2179|2591|2963|3389||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0107|0397|0719|1061|1439|1789|2203|2593|2969|3391||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0109|0401|0727|1063|1447|1801|2207|2609|2971|3407||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0113|0409|0733|1069|1451|1811|2213|2617|2999|3413||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0127|0419|0739|1087|1453|1823|2221|2621|3001|3433||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0131|0421|0743|1091|1459|1831|2237|2633|3011|3449||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0137|0431|0751|1093|1471|1847|2239|2647|3019|3457||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0139|0433|0757|1097|1481|1861|2243|2657|3023|3461||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0149|0439|0761|1103|1483|1867|2251|2659|3037|3463||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0151|0443|0769|1109|1487|1871|2267|2663|3041|3467||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0157|0449|0773|1117|1489|1873|2269|2671|3049|3469||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0163|0457|0787|1123|1493|1877|2273|2677|3061|3491||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0167|0461|0797|1129|1499|1879|2281|2683|3067|3499||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0173|0463|0809|1151|1511|1889|2287|2687|3079|3511||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0179|0467|0811|1153|1523|1901|2293|2689|3083|3517||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0181|0479|0821|1163|1531|1907|2297|2693|3089|3527||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0191|0487|0823|1171|1543|1913|2309|2699|3109|3529||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0193|0491|0827|1181|1549|1931|2311|2707|3119|3533||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0197|0499|0829|1187|1553|1933|2333|2711|3121|3539||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0199|0503|0839|1193|1559|1949|2339|2713|3137|3541||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0211|0509|0853|1201|1567|1951|2341|2719|3163|3547||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0223|0521|0857|1213|1571|1973|2347|2729|3167|3557||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0227|0523|0859|1217|1579|1979|2351|2731|3169|3559||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
|||||0229|0541|0863|1223|1583|1987|2357|2741|3181|3571||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
"#;

struct LineCollectorIODevice {}

impl IODevice for LineCollectorIODevice {
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        unimplemented!()
    }

    fn write(&mut self, data: &[FullWord]) -> Result<(), usize> {
        assert_eq!(data.len(), self.get_block_size());
        let mut writer = PRIMES_OUTPUT.write().unwrap();
        let mut count_written: usize = 0;
        // For each word...
        for word in data {
            // Each byte in a word...
            for &byte in &word[1..=5] {
                // Convert to char.
                let ch: char = Alphabet::try_from(byte)
                    .map_err(|_| count_written)?
                    .try_into()
                    .map_err(|_| count_written)?;
                if ch != ' ' {
                    *writer += format!("{}", ch).as_str();
                } else {
                    *writer += "|";
                }
                count_written += 1;
            }
        }
        *writer += "\n";
        Ok(())
    }

    fn control(&mut self, command: i16) -> Result<(), ()> {
        match command {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    fn is_busy(&self) -> Result<bool, ()> {
        Ok(false)
    }

    fn is_ready(&self) -> Result<bool, ()> {
        Ok(true)
    }

    fn get_block_size(&self) -> usize {
        24
    }
}

#[test]
fn primes() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.io_devices[18] = Some(Box::new(LineCollectorIODevice {}));

    // Test sequence: D. E. Knuth, 'The Art of Computer Programming',
    // Volume 1, pp 148.
    mix.mem[3000] = Instruction::new(0, 18, 0, Opcode::Ioc).into();
    mix.mem[3001] = Instruction::new(2051, 5, 0, Opcode::Ld1).into();
    mix.mem[3002] = Instruction::new(2050, 5, 0, Opcode::Ld2).into();
    mix.mem[3003] = Instruction::new(1, 0, 0, Opcode::Modify1).into();
    mix.mem[3004] = Instruction::new(499, 5, 1, Opcode::St2).into();
    mix.mem[3005] = Instruction::new(3016, 1, 0, Opcode::J1).into();
    mix.mem[3006] = Instruction::new(2, 0, 0, Opcode::Modify2).into();
    mix.mem[3007] = Instruction::new(2, 2, 0, Opcode::Modify3).into();
    mix.mem[3008] = Instruction::new(0, 2, 0, Opcode::ModifyA).into();
    mix.mem[3009] = Instruction::new(0, 2, 2, Opcode::ModifyX).into();
    mix.mem[3010] = Instruction::new(-1, 5, 3, Opcode::Div).into();
    mix.mem[3011] = Instruction::new(3006, 1, 0, Opcode::JX).into();
    mix.mem[3012] = Instruction::new(-1, 5, 3, Opcode::CmpA).into();
    mix.mem[3013] = Instruction::new(1, 0, 0, Opcode::Modify3).into();
    mix.mem[3014] = Instruction::new(3008, 6, 0, Opcode::Jmp).into();
    mix.mem[3015] = Instruction::new(3003, 0, 0, Opcode::Jmp).into();
    mix.mem[3016] = Instruction::new(1995, 18, 0, Opcode::Out).into();
    mix.mem[3017] = Instruction::new(2035, 2, 0, Opcode::Modify4).into();
    mix.mem[3018] = Instruction::new(-50, 2, 0, Opcode::Modify5).into();
    mix.mem[3019] = Instruction::new(501, 0, 0, Opcode::Modify5).into();
    mix.mem[3020] = Instruction::new(-1, 5, 5, Opcode::LdA).into();
    mix.mem[3021] = Instruction::new(0, 1, 0, Opcode::Special).into();
    mix.mem[3022] = Instruction::new(0, 12, 4, Opcode::StX).into();
    mix.mem[3023] = Instruction::new(1, 1, 0, Opcode::Modify4).into();
    mix.mem[3024] = Instruction::new(50, 1, 0, Opcode::Modify5).into();
    mix.mem[3025] = Instruction::new(3020, 2, 0, Opcode::J5).into();
    mix.mem[3026] = Instruction::new(0, 18, 4, Opcode::Out).into();
    mix.mem[3027] = Instruction::new(24, 5, 4, Opcode::Ld4).into();
    mix.mem[3028] = Instruction::new(3019, 0, 0, Opcode::J5).into();
    mix.mem[3029] = Instruction::new(0, 2, 0, Opcode::Special).into();

    mix.mem[0].set_all([0; 6]);
    mix.mem[1995].set_all([0, 6, 9, 19, 22, 23]);
    mix.mem[1996].set_all([0, 0, 6, 9, 25, 5]);
    mix.mem[1997].set_all([0, 0, 8, 24, 15, 4]);
    mix.mem[1998].set_all([0, 19, 5, 4, 0, 17]);
    mix.mem[1999].set_all([0, 19, 9, 14, 5, 22]);
    mix.mem[2024] = Word::<6, false>::from_i64(2035).0;
    mix.mem[2049] = Word::<6, false>::from_i64(2010).0;
    mix.mem[2050] = Word::<6, false>::from_i64(3).0;
    mix.mem[2051] = Word::<6, false>::from_i64(-499).0;

    mix.pc = 3000;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    let reader = PRIMES_OUTPUT.read().unwrap();
    assert_eq!(*reader, PRIMES_OUTPUT_EXPECTED);
}
