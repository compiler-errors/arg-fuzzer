
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5349(_: S1, _: S7, _: S8) {}
        
        fn test5349() { foo5349(S6, S1, S6, S2, S5, S5, S2); }
    