
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12782(_: S5, _: S4, _: S2) {}
        
        fn test12782() { foo12782(S1, S1, S5, S6, S5, S3, S4); }
    