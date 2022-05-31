
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12984(_: S4, _: S1, _: S7) {}
        
        fn test12984() { foo12984(S2, S3, S4, S5, S7, S8); }
    