
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5789(_: S8, _: S1, _: S1) {}
        
        fn test5789() { foo5789(S1, S2, S4, S5, S6, S8); }
    