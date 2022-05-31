
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15554(_: S1, _: S2) {}
        
        fn test15554() { foo15554(S5, S1, S4); }
    