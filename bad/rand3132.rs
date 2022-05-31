
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3132(_: S7, _: S6, _: S7) {}
        
        fn test3132() { foo3132(S1, S2, S4, S5, S6, S8); }
    