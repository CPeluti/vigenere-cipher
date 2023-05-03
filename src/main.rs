use std::{vec, collections::HashMap};
use regex::Regex;
const alphabet:&str = "abcdefghijklmnopqrstuvwxyz";
fn _cipher(text: &str, key: &str) -> String {
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

fn calculate_sd(freq: &Vec<(Vec<String>,usize,f64)>) -> f64{
    let mut average = 0.0;
    for (_,_, value) in freq {
        average += value;
    }
    average = average/(freq.len()) as f64;
    let mut sum = 0.0;
    for (_,_, value) in freq {
        sum += f64::powf((value-average) as f64,2.0);
    }
    return f64::sqrt(sum/(freq.len() as f64))
}

fn make_groups(text: &str, min_mod: usize, max_mod: usize) -> Vec<(Vec<String>,usize,f64)>{
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
        ioc_groups.push((groups,module, groups_ioc_med));
    }
    return ioc_groups;
}

fn chi_square(text: &String, letter_freq: HashMap<char, f64>) -> f64{
    
    let mut actual_frequency = vec![0;26];
    let chi_square_result = 0;
    for l in text.chars() {
        actual_frequency[l as usize - 97] += 1;
    } 
    let mut sum = 0.0;
    // For each letter in actual_frequency
    for (i,l) in actual_frequency.iter().enumerate() {
        //calculate expected value of that letter
        let expected = letter_freq[&char::from_u32((i+97) as u32).unwrap()] * text.len() as f64;
        // Calculate error
        let error = *l as f64 - expected;
        // Error^2
        let square = f64::powf(error, 2.0);
        sum += square/expected;
    }
    return sum;
}
fn solve(cosets: &Vec<String>, size:&usize, letter_freq: HashMap<char,f64>) -> String {
    let mut possible_password = String::new();
    for coset in cosets {
        let mut chi_square_scores = vec![];
        // apply shifts in the coset for each letter in the alphabet
        for i in 0..26{
            let mut shifted_string = String::new();
            for letter in coset.chars() {
                let new_letter = (((letter as i32 - 97) - i) + alphabet.len() as i32) % alphabet.len() as i32; 
                shifted_string.push(alphabet.chars().nth(new_letter.try_into().unwrap()).unwrap());
            }
            let score = chi_square(&shifted_string, letter_freq.clone());
            chi_square_scores.push(score);
        }
        let min = chi_square_scores.iter().enumerate().min_by(|(_, val_a),(_, val_b)| val_a.partial_cmp(val_b).unwrap()).unwrap();
        possible_password.push(char::from_u32(min.0 as u32 +97).unwrap());
    }
    return possible_password;
}
fn challenge(_letter_freq: HashMap<char,f64>, text: &str){
    let rgx = Regex::new(r"[^a-z]").unwrap();
    let text_formated = rgx.replace_all(text, "");

    // Make groups of mod X
    let  groups = make_groups(&text_formated,2,20);

    let dp = calculate_sd(&groups);

    let avg = groups.iter().fold(0.0, |acc, (_,_,i)| acc+i)/groups.len() as f64;
    let possible_keys: Vec<_> = groups.iter().filter(|(_, _,ic)| ic>=&(dp+avg)).map(|(key,size,_)| (key,size)).collect();
    for (cosets,possible_size) in possible_keys {
        let password = solve(cosets, possible_size,_letter_freq);
        println!("{:?}", password);
        break;
    } 
}

fn main() {
    let _letter_freq_pt_br = HashMap::from([
        ('a',14.634),
        ('b',1.043),
        ('c',3.882),
        ('d',4.992),
        ('e', 12.570),
        ('f',1.023),
        ('g',1.303),
        ('h',0.781),
        ('i',6.186),
        ('j',0.397),
        ('k',0.015),
        ('l',2.779),
        ('m',4.738),
        ('n',4.446),
        ('o',9.735),
        ('p',2.523),
        ('q',1.204),
        ('r',6.530),
        ('s',6.805),
        ('t',4.336),
        ('u',3.639),
        ('v',1.575),
        ('w',0.037),
        ('x',0.253),
        ('y',0.006),
        ('z',0.470)
    ]);
    let _letter_freq_eng = HashMap::from([
        ('a',8.167 ),
        ('b',1.492 ),
        ('c',2.782 ),
        ('d',4.253 ),
        ('e',12.702),
        ('f',2.228 ),
        ('g',2.015 ),
        ('h',6.094 ),
        ('i',6.966 ),
        ('j',0.153 ),
        ('k',0.772 ),
        ('l',4.025 ),
        ('m',2.406 ),
        ('n',6.749 ),
        ('o',7.507 ),
        ('p',1.929 ),
        ('q',0.095 ),
        ('r',5.987 ),
        ('s',6.327 ),
        ('t',9.056 ),
        ('u',2.758 ),
        ('v',0.978 ),
        ('w',2.360 ),
        ('x',0.150 ),
        ('y',1.974 ),
        ('z',0.074 )
        ]);
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
    let _desafio3 = "cvjtnafenmcdmkbxfstklhgsojwhofuisfyfbexeinfimayssdyyijnpwtokfrhwvwtzfxhluyumsgvdurbwbivxfafmyfyxpigbhwifhhojbexaunfiyljwdknhgaovbhhgvinaulzfofuqcvfbynftygmmsvgxcfzfokqatuifuferqtewzfokmwojylnzbkshoebpnaytfknxlbvuaxcxuyykytfrhrcfuycluktvgufqbeswysswlbyfefzvuwtrllngizgbmszkbtntslnnmdpmymiubvmtlobjhhfwtjnaufizmbzlivhmbsuwlbyfeuyfufenbrvjvkollgtvuzuaojnvuwtrlmbatzmfssojqxlfpknauljcioyvdrylujmvmlvmukbtnamfpxxjpdyfijfyuwsgviumbwstuxmssnykydjmcgasouxbysmcmeunfjnaufuyumwsfjukqwsvxxuvuffbpwbcfylwfdygukdrylujmfpxxefzqxyhgflacebjbxqstwiknmornxcjfaibwwbkcmukivqtmnbccthljyigimsycfvmurmayobjufvauzinmatcypbankbxlwjjnxujtwikbatcioybppzhlzjjzhllveyaifpllyijizmoudpllthvevumbxpibbmsnscmcgonbhckivlxmgcrmxnzbkqhodesytvgougthagrhrmhfreyijizgaunfziyzwouywqzpzmayjfjikovfkbtnoplfwhgusytlgnrhbzsopmiyslwikbanyuoyapwzxhvfuqaiatyykykpmceylirnpcdmeimfgwvbbmuplhmlqjwugskqvudzgsycfbswvchzxfexxxaqrolyxpiukyhmpnayfofhxbswvchzxfexxxairpxxgovhhggsvnhwsfjuknzbeshokirfexgufvkolvjnayivvmmcgofzackevumbatvhkidmvxbhlivwtjauffackhciksfpkyqnwolumyvxyykyaoyypukxflmbqoflackpwzxhufjygzgstywzgsnbbwzivmnzxfiywxwbkbayjftifykizmuivzdinlffuvrgssbugngopqailifozbzfyuwhgirhwcfizmwysuymaudmiyvyawvnaytfeyyclpwbbmvzzhzuhmrwxcfuyyvienfhpysmkbtmoizwaixzfolbsmchhnojkbmbatzxxjssknaulbjclfwxdsuykucioyjgflmbwhfiwixsfgxczbmymbwtrgxxshxykzgsdslydgnbxhaujbtfdqcytmwnpwhofuismiffvxfsvfrna";
    // let content = fs::read_to_string("./desafio1.txt").expect("file to be readable");

    
    let _result = challenge(_letter_freq_eng, _desafio3);
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}