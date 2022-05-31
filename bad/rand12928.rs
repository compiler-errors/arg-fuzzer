
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12928(_: S7, _: S5, _: S5) {}
        
        fn test12928() { foo12928(S1, S2, S4, S5, S6, S7, S8); }
    