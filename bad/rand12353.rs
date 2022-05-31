
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12353(_: S1, _: S2, _: S6, _: S7, _: S8) {}
        
        fn test12353() { foo12353(S1, S2, S1, S6, S5, S5, S3); }
    