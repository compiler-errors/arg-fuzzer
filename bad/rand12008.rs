
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12008(_: S1, _: S2, _: S3, _: S4, _: S5, _: S6) {}
        
        fn test12008() { foo12008(S5, S0, S6, S4, S3); }
    