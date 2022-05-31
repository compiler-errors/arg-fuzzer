
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12445(_: S2, _: S4, _: S1, _: S3, _: S5) {}
        
        fn test12445() { foo12445(S6, S1, S0, S7, S3, S7); }
    