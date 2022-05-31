
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16478(_: S6, _: S4, _: S7, _: S1) {}
        
        fn test16478() { foo16478(S7, S5, S0, S6, S0, S2); }
    