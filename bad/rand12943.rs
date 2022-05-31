
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12943(_: S6, _: S1) {}
        
        fn test12943() { foo12943(S2, S3, S4, S5, S6, S7, S8); }
    