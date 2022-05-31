
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12165(_: S6, _: S5, _: S4) {}
        
        fn test12165() { foo12165(S6, S1, S5, S1, S2); }
    