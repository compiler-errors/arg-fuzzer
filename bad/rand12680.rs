
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12680(_: S3, _: S6, _: S8) {}
        
        fn test12680() { foo12680(S5, S1, S3, S5, S1); }
    