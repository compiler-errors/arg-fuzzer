
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12415(_: S1, _: S5, _: S6) {}
        
        fn test12415() { foo12415(S1, S4, S5, S8); }
    