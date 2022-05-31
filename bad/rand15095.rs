
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15095(_: S3, _: S8) {}
        
        fn test15095() { foo15095(S4, S3, S7); }
    