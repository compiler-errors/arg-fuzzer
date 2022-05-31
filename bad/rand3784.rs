
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3784(_: S3, _: S4, _: S5, _: S1, _: S1, _: S2) {}
        
        fn test3784() { foo3784(S0, S3, S7, S3, S0, S1, S3); }
    