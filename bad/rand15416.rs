
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15416(_: S3, _: S4, _: S7) {}
        
        fn test15416() { foo15416(S2, S3, S6, S8); }
    