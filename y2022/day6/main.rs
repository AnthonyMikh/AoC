struct UniqueCounter {
    counts: [u8; Self::SIZE],
    n_unique: usize,
}

impl UniqueCounter {
    const SIZE: usize = 1 << u8::BITS;

    fn new() -> Self {
        Self {
            counts: [0; Self::SIZE],
            n_unique: 0,
        }
    }

    fn add(&mut self, ch: u8) {
        let count = &mut self.counts[ch as usize];
        if *count == 0 {
            self.n_unique += 1;
        }
        if *count == 1 {
            self.n_unique -= 1;
        }
        *count += 1;
    }

    fn remove(&mut self, ch: u8) {
        let count = &mut self.counts[ch as usize];
        if *count == 1 {
            self.n_unique -= 1;
        }
        *count -= 1;
        if *count == 1 {
            self.n_unique += 1;
        }
    }
}

fn solve<const UNIQ: usize>(s: &str) -> usize {
    let mut counter = UniqueCounter::new();
    let (first, rest) = s.as_bytes().split_at(UNIQ);
    for &ch in first {
        counter.add(ch);
    }
    if counter.n_unique == UNIQ {
        return UNIQ;
    }
    for (i, (&added, &removed)) in rest.iter().zip(s.as_bytes()).enumerate() {
        counter.add(added);
        counter.remove(removed);
        if counter.n_unique == UNIQ {
            return i + UNIQ + 1;
        }
    }
    unreachable!()
}

fn solve_packet(s: &str) -> usize {
    solve::<4>(s)
}

fn solve_message(s: &str) -> usize {
    solve::<14>(s)
}

fn main() {
    let input = "qvllndllhzhfzhhdzhddhjdjggvnvhvccmffwllqgqmmfjfqfhhtrrzczjczzl\
    plddfpptqqfbqffmnmjnnqppfjfccgnnmqqsvvdbbgppjvpjvpjjctjjttwtrrdldlcddrvddqn\
    dqnqwqwzwfwwzczggcppgzpzhpzhppprfffbhhwmhhtftstrsrvsrvsrvvshvssnwwpllhfhnnf\
    flcltlblzlqlvqlvlcldcccpptggtdgdjdbbrggmbmnncscbssqrrjddvcvgvfflpppgpvphvph\
    hpcpzpzvvctvctvthtwtfwwbrrhhlplmlwwlqlnlhhtmhmmqlqplllrvrgvrvrffzfgfjfjtjmj\
    vmjmwmvvjffmpfphfhvfvmfvmmhpphhltthgttgccqggpzpfpqpcpvcpvcvvqtqvqbbrlrtllmr\
    mllhmhvmhhvzhvzvrrrzjzbbtvvbgvbbfnnqndqnnpnbnbnlnggwqggmgmqmgmbbmccgqcqbccp\
    vcvnnhvvrvlrrcwrcwrcwrwbrwwzbwbdbfddpttntzzjszsnznbndnzngzgccjrcjchcffmlmqq\
    lrqqzsqzzsbsnsttzpztpzpggzrrttbqqplpqlqjjqcqvccdzdccthccvfcvvqvhqhfhhzwzpzw\
    ppgpttntssflfjjrwrqrjrppptlltptpvttpfpwpswpppzzsrzssqllbnlljpllrjllsrlrhrdr\
    mdrmrrpsrprnrffgrffdqdhdqhhrhggwqqlddsbsqbqtqdtdhdvhhbdhdzhdhhtrrppzddgfgzg\
    pzpvpfpnpptggltggbnbppqffzfrzzzsbsrrdgrddwsdsqddhpdpbpvpfvppfsfgfngffzmzbzl\
    blclsccvqvqmmjtjqtjjlcjllsddjqddhldlvlrrbgbrgbrrdzzpfpggqnqbqrrqbbgjgppqgpg\
    wgqqndncndnpdnnbvbnvnwnjjgppzlplqqdgqghqgqzggjssqmmwwcfcpptrpprggrppgbplmzw\
    mdtnpqwzcrthqbppwbgcvgqrpfpnbscnhvrllpvpqwnsslcjrqtvdccprvqfrpswtpvzdzlgtmm\
    vppdmhgdbbsmrbqpqspdhpqgfjznqzphrnggcbzhdqrgvzcfzrhtrlssgmjjghqsjtghhnwjffq\
    rrfslfnsvvdvfjqbfpffrrstdhggvbfwtfpfgswqlfdrnjpjmwzptlbmwgghgwqrphcrvfmhrpl\
    llgbnjlprllmjwccphsflntgpnbmdbfqcdsbgvrnfznfrlcfvswqfrqvdnbjsflnsmlcrdstzpp\
    mcvbgdtcvgztbdzqbwhmwcfvbwjjcdgbnwjwzrrdqhpgscwtnztjsfstzfwftcldjgvdvwbzrlb\
    dslwttbqpnlwbjcjwqgtrgcglsgtdqbqbnqznptzzbwffwlwzvvtdpcjbvhnswzptclpbndcdvs\
    fmcrmwwgzdfsszqjjdztmtsqgfqzjpctfdpwnzbpnzzwngqnghntblndfrnjzdrmgbqmzbdqfzc\
    trgshwqgfgqssqjltrqlzjswjhmpgwwjdwcjpnsvgrvbfpmlmmwzmbdjwsrjthppfrccjgnmwlv\
    qlprgslbwtbbzlqbznczmsmhsfdcqnwblprcpbzzwfllbnldvpjcwsdhglrzjsptmsjdjqzsmgv\
    hjfjrrtvvbjlmzjsntnrggwbpjlrjggfgqzvswtggthzfmfjnmrzrttbzqpwpsnmdtnbfblpfgs\
    lgcmjlbdpshnnrbhvwsbrnvdmjqhvhdjhbfzjmqrmqmdthhzvnrmqcnbtwcdjdqfvdgvmfbhrfq\
    nmdncrddggtcppjlznbsnntppjtnsqsrjwvfrzpnzqcrzhhdflfmmtmwcvtpzbqhdwsczffcqht\
    dbdjblmgnrmhlqcsvcpgghhvwqhdtzpzlpfllchzltqgcwgfqnbzhgzmdwqdlwnvhqmpqjqnjbh\
    jctslghdqvctdmjfwdfpdjnhdndzwsfjzlmsbmfmzvnvpqgqhtngvgqmlrrzsfmwlcwsscvghjv\
    rzjjqbnplnjzqswpblwzwczhwbhhnjmctnmwlbqqfmnlwdcrptlmfjpjrnpcvmhffjhwhmntdzp\
    djzwzhrrsdvmjlwdtcpvjfmfzfsrgjghhlvmjjjczgmhvrfpgqbnhldwbrjgzmnszzbssfzcggr\
    wmdfvddwsdmnwtwfwlfnwlvzlctfblbtrjvcwjjdljplcrjhwqslppwwtvfqwsjlfmdznmcdzdm\
    gvmmsrfcclcvhtrhlsjzrbjwrjlfnvqhqvmpzmdttnbhfcvnqlrqbcsvtvwfccjstjpmhqgwlnr\
    zjjmfdszflmglrdbpqhqhqsdfzrcljbdvvnlcqfllmnqcjfzjppdsjwshfschzqbnwfqnpwhqnm\
    wsjbtcgvrljsrtzvcvghcjjlqsngglcggqpntrrhbjpbfhmvpltmnfmfdtwnczwfbvjcqnhvppj\
    ftwvwsrlhvvcjtsfptpqgrmrqwwddnqmnmfgrlnphbpqhhhvglqgtwvnwvnbssftmwttmfrffwt\
    zhrpqspclvgchwqwcsgwqwwvpgcwngrcfmhbhflwfbfchlphdzdcrflfmfclsngtlwrqcrsgrdz\
    cpdsvvcdbhgtljmbntbbcqgjqfsbfwzlfsnljpjdcnmjlqrwpmlvwgdlrrdgfhdqhzgltmclzgz\
    zhmrbggsmgtpqdrgmjtlzwstrwbpvhppvsmdqvvwwglzjgdswjszqmrdbmshbhhcstpcsjdbvgj\
    nvcmvhbtclrlmlgnvppgvncsrfchdbqjrclwwlnchmcgvshfsbsvvcvjrsgjlnsfqtqmgntffwn\
    qjtldcqbcqhsgztllstswwqnfrswpchqhnfzzzszqjztzfrgrbjdbjlpvqfqrlrmmpbfbbcclrg\
    mnlzwqrjhqrstswjpgsrtnlwsbqthzpvdzllzqmdmbvvtcztftvlwphhjzbfnrvccfmhmvmzlbr\
    zlnppfzcsffjvjmbgpvlwgwszpztjpsrbnftqtdrbnljtbrjzzbwlsvtwtlwptdtnmtncvcblcm\
    dngjzmctlqtzchncccnwjzrrmmmnllbhrnhwtqjsnvcslrqjfbfndqvdlrjshdzmlprtzbtnhth\
    dqhplwzdbnjmgzlzrbzrvrqnflwfmsmbssqnbcddnvdpltpmplpdzvtjrslcdcnrdplwtjtvctw\
    fzhlvwwqqtbqcjjwhhnpmvgzhqmqfgthwbphrmrtdghchsmwghdqjgjgmpddbrtngtvhqgjfrpl\
    rdgpbnhqvswrmqhcmsqvsqmqsgwjndwjrbrhvrctmmrmfwpsgfgdlrzpslpflgvwrgcthgcrnhg\
    rzsmqdgdssjgspfhmqfmjfpmwqhnfjdvqzhpndvnbmqglbrjmdrwgmgctrgzpsdvfbmcstcslbl\
    mvnprphntgslmlrqwthrndrhtbccgzzfsglhgqztcsnqjwfzbzlvrpbvswbhrwdsrhrrpnrmsbv\
    bvjccbdsdcfrrzpgwjtnnnvjwlcppwzdqsbdzpfjplrlfgvjpsmbzwpwlghnvqgddfjvrsztrpz\
    lfgmqqzrfcgglghndbhgbmldglclhldljjdslvhzshshtqwhqnbzhvqrcmwdmcmhjcrmdmhrwnw\
    cbhvbbrwrbtfdnztwnbpdfjfhgrmcpngftsvbsmsptnwcvvllnmbnsntbzmwnhfdptbtzswtjzd\
    qwjdhprnjwvhzpscjvlsgrhdrmmrmhzhwwtslzdjqmzfncnmgplhnmwrvqhslvchtjcmpzpjpnp\
    fbjptvvwcsmhgdjtsqrjlfpnfdncpqqmpgpvtlvwljlsqbnhtsqgfwlsmdjpgtvgjvjcrnnzmbl\
    lqzlrfdnlffgmtphhhgbcjgdlpzqpwmjwtcmdrsmtnmddftwczbsddtppsptbwfvpnfnsqmsgcf\
    qfmnzffzqgcdvwzrgdwhmnzmrlhcdpdsltnsmjzdqwmmpwvjqbbwsrfgzh";
    println!("{}", solve_packet(input));
    println!("{}", solve_message(input));
}
