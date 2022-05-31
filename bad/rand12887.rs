
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12887(_: S3, _: S6, _: S2, _: S4) {}
        
        fn test12887() { foo12887(S1, S3, S5, S6, S8); }
    