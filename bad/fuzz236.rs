
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo236(_: S1, _: S8) {}
        
        fn test236() { foo236(S5, S1, S3); }
    