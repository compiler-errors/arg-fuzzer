
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12214(_: S8, _: S6, _: S2, _: S4) {}
        
        fn test12214() { foo12214(S3, S2, S5, S1, S6, S8, S7); }
    