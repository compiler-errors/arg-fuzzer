
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5284(_: S4, _: S5, _: S7) {}
        
        fn test5284() { foo5284(S2, S1, S4, S8, S5, S6); }
    