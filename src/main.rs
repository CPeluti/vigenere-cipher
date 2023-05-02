use std::{vec};
use regex::Regex;

fn _cipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut ciphred_text = String::from("");
    for (i,letter) in text.chars().enumerate() {
        // println!("{}", letter as u32-97);
        let letter_value = letter as u32-97;
        let key_letter = key.as_bytes()[i%key.len()] as u32-97;
        let new_index = (letter_value+key_letter)%(alphabet.len() as u32);
        let ciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
        ciphred_text.push(ciphred_letter);
    }
    return ciphred_text;
}

fn _decipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut deciphred_text = String::from("");
    let mut counter_non_chars = 0;
    for (i, letter) in text.chars().enumerate() {
        if ![' ', '\n', ';', '\'', '—','-',',','.'].contains(&letter){
            let index = i-counter_non_chars;
            let let_value = letter as i32-97;
            let key_let = key.as_bytes()[index%key.len()] as i32-97;
            let new_index = (let_value-key_let + (alphabet.len()) as i32) % (alphabet.len()) as i32;
            
            let deciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
            deciphred_text.push(deciphred_letter);
        } else {
            deciphred_text.push(letter);
            counter_non_chars+=1;
        }
    }
    return deciphred_text;
}
fn ioc(text: &str) -> f64{
    let mut letter_freq_in_text = vec![0; 26];
    for i in text.chars() {
        letter_freq_in_text[(i as usize)-97] += 1;
    }
    let n = text.len();
    let mut sum = 0;
    for freq in letter_freq_in_text.iter() {
        sum += freq*(freq-1);
    }
    let denominator = n*(n-1);
    return 26.0*sum as f64/denominator as f64;

}

// fn calculate_lmc(n: i32) -> Vec<i32>{
//     let mut lmc: Vec<i32> = vec![];
//     for i in 2..30 {
//         if n % i == 0 {
//             lmc.push(i);
//         }
//     }
//     return lmc;
// }

fn calculate_sd(freq: &Vec<(usize,f64)>) -> f64{
    let mut media = 0.0;
    for (_, value) in freq {
        media += value;
    }
    media = media/(freq.len()) as f64;
    let mut sum = 0.0;
    for (_, value) in freq {
        sum += f64::powf((value-media) as f64,2.0);
    }
    return f64::sqrt(sum/(freq.len() as f64))
}

fn make_groups(text: &str, min_mod: usize, max_mod: usize) -> Vec<(usize,f64)>{
    // loop for each module
    let mut ioc_groups = vec![];
    for module in min_mod..=max_mod{
        let mut groups = vec![];
        // Initialize N strings for each group
        for _ in 0..module {
            groups.push(String::new());
        }
        for (index, char) in text.chars().enumerate(){
            groups[index%module].push(char);
        }
        let iocs = groups.iter().map(|s| ioc(s)).collect::<Vec<f64>>();

        let mut groups_ioc_med = iocs.iter().sum();

        groups_ioc_med = groups_ioc_med/iocs.len() as f64;
        ioc_groups.push((module, groups_ioc_med));
    }
    return ioc_groups;
}
fn challenge(_letter_freq: &str, text: &str){
    let rgx = Regex::new(r"[^a-z]").unwrap();
    let text_formated = rgx.replace_all(text, "");

    // Make groups of mod X
    let  groups = make_groups(&text_formated,2,20);

    let dp = calculate_sd(&groups);

    let avg = groups.iter().fold(0.0, |acc, i| acc+i.1)/groups.len() as f64;
    let possible_keys: Vec<_> = groups.iter().filter(|(_, ic)| ic>=&(dp+avg)).map(|(key,_)| key).collect();
    println!("{:?}", possible_keys)
    
    // let  mut dict_tri: HashMap<String, (i32, Vec<i32>)> = HashMap::new();
    // for i in 0..(text_formated.len()-3) {
    //     let mut trigrama = String::new();
    //     trigrama.push(text_formated.chars().nth(i).unwrap());
    //     trigrama.push(text_formated.chars().nth(i+1).unwrap());
    //     trigrama.push(text_formated.chars().nth(i+2).unwrap());
    //     trigrama.push(text_formated.chars().nth(i+3).unwrap());

    //     dict_tri.entry(trigrama).and_modify(|(value, vector)| {
    //         *value += 1;
    //         vector.push(i as i32);
    //     }).or_insert((1, vec![i as i32]));
    // }
    // let dict = dict_tri.iter().filter(|&(s, (value, _))| *value == 2);
    // let dict_debug:HashMap<_,_> = dict.clone().collect();
    // let mut dict_freq = HashMap::new();
    // for (key, (val, pos)) in dict {
    //     // if *val>=2 {
    //         // Spacing between trigramas
    //         let sub = pos[1] - pos[0];
    //         // Calculate divisors of lmc
    //         let divisores: Vec<i32> = calculate_LMC(sub);
    //         // Count Number of occurrence of each divisor
    //         for i in divisores {
    //             dict_freq.entry(i).and_modify(|value| *value += 1).or_insert(1);
    //         }
    //     // }
    // }
    // // dict_freq.iter().max_by(|(a, b)| a.1)

    // let mut key_len = (0, 0);
    // let mut dp = calculate_DP(dict_freq.clone());
    // for (key, value) in dict_freq {
    //     if value >= key_len.1 {
    //         if value == key_len.1 {
    //             if key_len.0 > key {
    //                 key_len = (key, value);
    //             }
    //         } else {
    //             key_len = (key, value);
    //         }
    //     }
    // }
    // println!("{:?}",key_len);
}

fn main() {
    let _letter_freq_pt_br = "aeosrindmutclpvghqbfzjxkwy";
    let letter_freq_eng = "etaoinshrdlcumwfgypbvkjxqz";
    let _desafio = "rvgllakieg tye tirtucatzoe.  whvnvvei i
    winu mpsecf xronieg giid abfuk thv mfuty; wyenvvvr ik ij a drmg,
    drzzqly eomemsei in dy jouc; wyenvvvr i wied mpsvlf znmollnkarzlp
    palszng seworv cfffzn narvhfusvs, rnd srzngznx up khv rerr ff emeiy
    flnvrac i deek; aed ejpvcirlcy wyeeevvr dy hppfs gvt jucy ae upgei
    haed ff mv, tyat zt ieqliies r skroeg dorrl grieczplv tf prvvvnt de
    wrod dvliseiatvlp stvpginx ieto khv stievt, aed detyouicrlcy keotkieg
    geoglv's hrtj ofw--tyen, z atcolnk it yixh tzmv to xek to jer as jofn
    aj i tan.  khzs ij mp susskitltv foi pzstfl rnd sacl.  wzty a
    pyicosfpyicrl wlolrzsh tako tyrfws yidsecf lpoe hzs snoid; i huzetcy
    kakv tf thv syip.  khvre zs eotyieg slrgrijieg ie tyis.  zf khep blt
    keen it, rldosk acl mvn zn tyezr dvgiee, jode tzmv or ftyer, thvrijh
    merp nvarcy khe jade fvecinxs kowrrus tye fcern nity mv.";
    let _desafio2 = "tpsja kexis ttgztpb wq ssmil tfdxf vsetw ytafrttw btzf pcbroxdzo zn tqac wix, bwfd s, je ahvup sd pcbqqxff lfzed d avu ytwoxavneh sg p aznst qaghv. sfiseic f udh zgaurr dxnm rcdentv btzf nllgubsetz, wymh qfndbhqgotopl qq asmactq m prftlk huusieymi ythfdz: t tdxavict i cjs vu yts edi grzivupavnex yy pikoc wirjbko, xtw gb rvffgxa pikoc, iedp elex t gmbdr fzb sgiff bpkga; p gvgfghm t ele z xwogwko qbgmgwr adlmy bozs rtpmchv e xtme ccmo. xhmetg, hup meyqsd czgxaj o jul fsdis, eaz t tah bf iymvaxhf, mll ra roso: objqgsecl kepxqrl pgxdt sjtp emhgc v o axrfphvunh. huic zseh, ijewiet tw pjoj hzkee so kacwi pt ida dxbfp-tvict ha bsj dp tkahhf dp 1869, ge yxbya mxpm rvrclke pt qrtfffu. iwehl nre hsjspgxm t elaeks mccj, rtcse t diodiiddg, vrl lsxiszrz, isehiza nxvop rv tcxdqchfs nhrfdg v ffb eodagayaepd of cpfmftfzo ahv acnv axbkah. cezp tquvcj! vpkhmss v qfx rmd vfugx gmghrs yxq mciecthw. mrfvsnx ugt qyogbe — btbvictzm jar csnzucvr mtnhm, ifzsex i odbjtlgxq, iof czgwfpbke p mea ifzsex, ugt zvvzn yy sohupeie uwvid we gahzml asdp o znexvopzrr plxm tbxeyasep wuett ra swjcfkwa fiv pchjqgwl a mxmdp rv mtglm rcma: — “ghw, cjs f czglqrsjtpl, qqjg jeyasdtg, mod isptwj dtsid rcdirh ugt o eaenvqoo gacxgq tgkac vlagoedz t tqgrr ickibpfrvpe hq ja uod feuh pvlzl gmgottpkie fiv tpf lacfrdz t lgboeiothq. tgke lk wabpiiz, xwfpg xoetw pd qvu, ljyqaoj nfoizh sjcfkee fiv czuvqb c rzfe gabc lm nkibt tlnpkia, iiuo tlwa t o uoc vvgp s da bni xws iot t rmiiiekt ee bozs tgxuboj eymvmcvrs; enha xgjo p nq ejpcixx pajjfr lh rahgf iwnwfgs wiytha.” qcd e qbix pazgz! gea, cof mp tvdtdvnoh hmh jznex ebdzzcpl ugt zye oxmjtw. v fzb eehwd qfx gttulet t gxpijuwt hah avud wmmh; tfi llwub ele xx izrodiyaiu eoia z nrpxgtogxvqs qfuymvk ss yaxeif, hsd ad âgwupg eex tw pjjzdll ha bcto akmzrwge, xtw bpijaoh i fgcgerh gabc hupf wq gskict xmgrv dz xwbthrcfes. fpfue p tfagfvctws. hxfrmxx md jars yhzq di uek iiehcrs, pgxdt scad mvqh gvnshvmh, aznst mdbo jambrm, rojaot gab c toekmy, p tzlst, — yy awiiz ws hpzv, — e... exrtpa ganbizrwr! dljyu p dfunh pttg uicxm cjsd ect e ftftetke etbyoct. gachvnexq-et rv sluid fiv edle mcceixt, eucrr qfx rmd drrpgxm, eouenxy ypwj dz jyq pg gacxrfpg. v vpkhmss, gaoxgqj arid. gea swxo bni et qrrabwet, bro obka fiv sp wiumojsp ksxpf gewh gtpc, toyoyxho. eex h qqj csieh idp qfidt exiodeymi pgodaebgm... ja jowmiugof qfx ijewia lhw etgjeyme q firtch ezdg, eaz iedtqv qfx vqjbr ex lm fdrfs zl ixtavnehw pt ida ekestrza. p wepd ele dbq, a fiv mpgse rcevtglm p sjsl tracwda pke meoieyme-xd. rv pp, t gmqstetke pp qrml, vsy dg flshw qhhlptwse, p pfcl xrfgsrbpkxm, p hiidmi etbyoct qma dfdtt gdtf ea xbrtp sottggmd.";
    let desafio3 = "cvjtnafenmcdmkbxfstklhgsojwhofuisfyfbexeinfimayssdyyijnpwtokfrhwvwtzfxhluyumsgvdurbwbivxfafmyfyxpigbhwifhhojbexaunfiyljwdknhgaovbhhgvinaulzfofuqcvfbynftygmmsvgxcfzfokqatuifuferqtewzfokmwojylnzbkshoebpnaytfknxlbvuaxcxuyykytfrhrcfuycluktvgufqbeswysswlbyfefzvuwtrllngizgbmszkbtntslnnmdpmymiubvmtlobjhhfwtjnaufizmbzlivhmbsuwlbyfeuyfufenbrvjvkollgtvuzuaojnvuwtrlmbatzmfssojqxlfpknauljcioyvdrylujmvmlvmukbtnamfpxxjpdyfijfyuwsgviumbwstuxmssnykydjmcgasouxbysmcmeunfjnaufuyumwsfjukqwsvxxuvuffbpwbcfylwfdygukdrylujmfpxxefzqxyhgflacebjbxqstwiknmornxcjfaibwwbkcmukivqtmnbccthljyigimsycfvmurmayobjufvauzinmatcypbankbxlwjjnxujtwikbatcioybppzhlzjjzhllveyaifpllyijizmoudpllthvevumbxpibbmsnscmcgonbhckivlxmgcrmxnzbkqhodesytvgougthagrhrmhfreyijizgaunfziyzwouywqzpzmayjfjikovfkbtnoplfwhgusytlgnrhbzsopmiyslwikbanyuoyapwzxhvfuqaiatyykykpmceylirnpcdmeimfgwvbbmuplhmlqjwugskqvudzgsycfbswvchzxfexxxaqrolyxpiukyhmpnayfofhxbswvchzxfexxxairpxxgovhhggsvnhwsfjuknzbeshokirfexgufvkolvjnayivvmmcgofzackevumbatvhkidmvxbhlivwtjauffackhciksfpkyqnwolumyvxyykyaoyypukxflmbqoflackpwzxhufjygzgstywzgsnbbwzivmnzxfiywxwbkbayjftifykizmuivzdinlffuvrgssbugngopqailifozbzfyuwhgirhwcfizmwysuymaudmiyvyawvnaytfeyyclpwbbmvzzhzuhmrwxcfuyyvienfhpysmkbtmoizwaixzfolbsmchhnojkbmbatzxxjssknaulbjclfwxdsuykucioyjgflmbwhfiwixsfgxczbmymbwtrgxxshxykzgsdslydgnbxhaujbtfdqcytmwnpwhofuismiffvxfsvfrna";
    // let content = fs::read_to_string("./desafio1.txt").expect("file to be readable");

    
    let _result = challenge(letter_freq_eng, desafio3);
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}
