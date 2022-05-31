
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12754(_: S5, _: S7) {}
        
        fn test12754() { foo12754(S1, S2, S3, S5, S6); }
    