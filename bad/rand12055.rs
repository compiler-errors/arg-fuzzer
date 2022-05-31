
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12055(_: S1, _: S5, _: S3) {}
        
        fn test12055() { foo12055(S7, S1, S3, S3, S6, S2); }
    