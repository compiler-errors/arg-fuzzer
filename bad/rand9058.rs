
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9058(_: S5, _: S7) {}
        
        fn test9058() { foo9058(S1, S8, S5, S3, S2, S6); }
    